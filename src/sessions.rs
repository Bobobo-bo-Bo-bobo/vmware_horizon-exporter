use crate::configuration;
use crate::constants;
use crate::data;
use crate::exporter;
use crate::globals;
use crate::horizon;

use lazy_static::lazy_static;
use log::{debug, warn};
use std::collections::HashMap;
use std::error::Error;
use std::sync::Mutex;

// Map pool id -> state, count
type SessionMap = HashMap<String, HashMap<String, i64>>;

// Map pool id -> agent, count
type AgentVersionMap = HashMap<String, HashMap<String, i64>>;

// Map poold id -> session protocol, count
type SessionProtocolMap = HashMap<String, HashMap<String, i64>>;

// Map pool id -> session type, count
type SessionTypeMap = HashMap<String, HashMap<String, i64>>;

fn flush_session_type_map(m: &mut SessionTypeMap) {
    for (k1, v1) in m.iter_mut() {
        for (k2, v2) in v1.iter_mut() {
            debug!(
                "sessions.rs:flush_session_type_map: setting m[{}][{}] from {} to 0",
                k1, k2, *v2
            );
            *v2 = 0;
        }
    }
}

fn flush_agent_version_map(m: &mut AgentVersionMap) {
    for (k1, v1) in m.iter_mut() {
        for (k2, v2) in v1.iter_mut() {
            debug!(
                "sessions.rs:flush_agent_version_map: setting m[{}][{}] from {} to 0",
                k1, k2, *v2
            );
            *v2 = 0;
        }
    }
}

fn flush_session_protocol_map(m: &mut SessionProtocolMap) {
    for (k1, v1) in m.iter_mut() {
        for (k2, v2) in v1.iter_mut() {
            debug!(
                "sessions.rs:flush_session_protocol_map: setting m[{}][{}] from {} to 0",
                k1, k2, *v2
            );
            *v2 = 0;
        }
    }
}

fn initialise_session_protocol_map(m: &mut SessionProtocolMap, p: &str) {
    debug!(
        "sessions.rs:initialise_session_protocol_map: initialising SessionProtocolMap for {}",
        p
    );

    let pm = m
        .entry(p.to_string())
        .or_insert_with(HashMap::<String, i64>::new);
    pm.insert(constants::LC_PROTOCOL_PCOIP.to_string(), 0);
    pm.insert(constants::LC_PROTOCOL_RDP.to_string(), 0);
    pm.insert(constants::LC_PROTOCOL_BLAST.to_string(), 0);
    pm.insert(constants::LC_PROTOCOL_CONSOLE.to_string(), 0);
    pm.insert(constants::LC_PROTOCOL_UNKNOWN.to_string(), 0);
}

fn initialise_session_type_map(m: &mut SessionTypeMap, p: &str) {
    debug!(
        "sessions.rs:initialise_session_type_map: initialising SessionTypeMap for {}",
        p
    );

    let pm = m
        .entry(p.to_string())
        .or_insert_with(HashMap::<String, i64>::new);
    pm.insert(constants::LC_TYPE_APPLICATION.to_string(), 0);
    pm.insert(constants::LC_TYPE_DESKTOP.to_string(), 0);
}

fn flush_session_map(m: &mut SessionMap) {
    for (k1, v1) in m.iter_mut() {
        for (k2, v2) in v1.iter_mut() {
            debug!(
                "sessions.rs:flush_session_map: setting m[{}][{}] from {} to 0",
                k1, k2, *v2
            );
            *v2 = 0;
        }
    }
}

fn initialise_session_map(m: &mut SessionMap, p: &str) {
    debug!(
        "sessions.rs:initialise_session_map: initialising SessionMap for {}",
        p
    );

    let pm = m
        .entry(p.to_string())
        .or_insert_with(HashMap::<String, i64>::new);
    pm.insert(constants::LC_SESSION_CONNECTED.to_string(), 0);
    pm.insert(constants::LC_SESSION_DISCONNECTED.to_string(), 0);
    pm.insert(constants::LC_SESSION_PENDING.to_string(), 0);
}

pub fn session_metric_update(
    cfg: &configuration::Configuration,
    client: &mut reqwest::blocking::Client,
    token: &str,
) -> Result<(), Box<dyn Error>> {
    lazy_static! {
        static ref POOL_SESSIONS: Mutex<SessionMap> = Mutex::new(HashMap::new());
        static ref AGENT_VERSIONS: Mutex<AgentVersionMap> = Mutex::new(HashMap::new());
        static ref POOL_PROTOCOLS: Mutex<SessionProtocolMap> = Mutex::new(HashMap::new());
        static ref TYPES: Mutex<SessionTypeMap> = Mutex::new(HashMap::new());
    }
    let mut pool_sessions = POOL_SESSIONS.lock().unwrap();
    let mut agent_versions = AGENT_VERSIONS.lock().unwrap();
    let mut pool_protocols = POOL_PROTOCOLS.lock().unwrap();
    let mut types = TYPES.lock().unwrap();

    // flush existing counters to prevent reporting of stale data
    flush_session_map(&mut pool_sessions);
    flush_agent_version_map(&mut agent_versions);
    flush_session_protocol_map(&mut pool_protocols);
    flush_session_type_map(&mut types);

    let dsktp_pools = globals::DESKTOP_POOLS.lock().unwrap().clone();

    for dp in dsktp_pools {
        if cfg.horizon_api.skip_pools_set.contains(&dp.id) {
            continue;
        }
        if !cfg.horizon_api.only_pools_set.is_empty()
            && !cfg.horizon_api.only_pools_set.contains(&dp.id)
        {
            continue;
        }
        if !pool_sessions.contains_key(&dp.id) {
            initialise_session_map(&mut pool_sessions, &dp.id);
            initialise_session_protocol_map(&mut pool_protocols, &dp.id);
            initialise_session_type_map(&mut types, &dp.id);
        }
    }

    debug!("sessions.rs:session_metric_update: getting list of current sessions");
    let sessions = horizon::get_sessions(cfg, client, token)?;

    for s in sessions.iter() {
        debug!(
            "sessions.rs:session_metric_update: processing session data - {:?}",
            s
        );
        if let Some(dp_id) = &s.desktop_pool_id {
            if cfg.horizon_api.skip_pools_set.contains(dp_id) {
                debug!(
                    "sessions.rs:session_metric_update: desktop pool id {} is in skip_pools list",
                    dp_id
                );
                continue;
            }

            if !cfg.horizon_api.only_pools_set.is_empty()
                && !cfg.horizon_api.only_pools_set.contains(dp_id)
            {
                debug!("sessions.rs:session_metric_update: only_pools list is not empty and desktop pool id {} is not in only_pools list", dp_id);
                continue;
            }

            set_desktop_pool_session_metrics(&mut pool_sessions, s, dp_id);
            set_agent_version_metrics(&mut agent_versions, s, dp_id);
            set_desktop_pool_session_protocol_metrics(&mut pool_protocols, s, dp_id);
            set_desktop_pool_session_type_metrics(&mut types, s, dp_id);
        } else {
            warn!("BUG: session id {} is not a desktop session", s.id);
        }
    }

    prometheus_pool_sessions(&pool_sessions, &cfg.horizon_api);
    prometheus_agent_versions(&agent_versions, &cfg.horizon_api);
    prometheus_pool_session_protocols(&pool_protocols, &cfg.horizon_api);
    prometheus_pool_session_types(&types, &cfg.horizon_api);

    Ok(())
}

fn prometheus_agent_versions(amap: &AgentVersionMap, cfg: &configuration::HorizonAPIConfig) {
    for (pool, vcount) in amap.iter() {
        for (ver, count) in vcount.iter() {
            exporter::AGENT_VERSIONS
                .with_label_values(&[&cfg.clone().user_defined_pool_uuid_resolve(pool), ver])
                .set(*count);
        }
    }
}

fn prometheus_pool_sessions(pmap: &SessionMap, cfg: &configuration::HorizonAPIConfig) {
    for (pool, scount) in pmap.iter() {
        for (state, count) in scount.iter() {
            exporter::SESSIONS
                .with_label_values(&[&cfg.clone().user_defined_pool_uuid_resolve(pool), state])
                .set(*count);
        }
    }
}

fn prometheus_pool_session_protocols(
    pmap: &SessionProtocolMap,
    cfg: &configuration::HorizonAPIConfig,
) {
    for (pool, scount) in pmap.iter() {
        for (proto, count) in scount.iter() {
            exporter::SESSION_PROTOCOLS
                .with_label_values(&[&cfg.clone().user_defined_pool_uuid_resolve(pool), proto])
                .set(*count);
        }
    }
}

fn prometheus_pool_session_types(pmap: &SessionTypeMap, cfg: &configuration::HorizonAPIConfig) {
    for (pool, scount) in pmap.iter() {
        for (_type, count) in scount.iter() {
            exporter::SESSION_TYPES
                .with_label_values(&[&cfg.clone().user_defined_pool_uuid_resolve(pool), _type])
                .set(*count);
        }
    }
}

fn set_desktop_pool_session_type_metrics(smap: &mut SessionTypeMap, s: &data::Session, id: &str) {
    match s.session_type.as_str() {
        constants::TYPE_APPLICATION | constants::TYPE_DESKTOP => {}
        _ => {
            warn!(
                "skipping unknown state {} for session id {}",
                s.session_state, s.id
            );
        }
    };

    let ps = smap
        .entry(id.to_string())
        .or_insert_with(HashMap::<String, i64>::new);
    let lc_type = s.session_type.to_lowercase();

    *ps.entry(lc_type).or_insert(0) += 1;
}

fn set_desktop_pool_session_protocol_metrics(
    smap: &mut SessionProtocolMap,
    s: &data::Session,
    id: &str,
) {
    if let Some(v) = s.session_protocol.clone() {
        match v.as_str() {
            constants::PROTOCOL_PCOIP
            | constants::PROTOCOL_RDP
            | constants::PROTOCOL_BLAST
            | constants::PROTOCOL_CONSOLE
            | constants::PROTOCOL_UNKNOWN => {}
            _ => {
                warn!(
                    "skipping unknown session protocol {} for session id {}",
                    v, s.id
                );
            }
        };
        let vl = v.to_lowercase();
        let ps = smap
            .entry(id.to_string())
            .or_insert_with(HashMap::<String, i64>::new);
        *ps.entry(vl).or_insert(0) += 1;
    }
}

fn set_agent_version_metrics(amap: &mut AgentVersionMap, s: &data::Session, id: &str) {
    let sm = amap.entry(id.to_string()).or_insert_with(HashMap::new);
    *sm.entry(s.agent_version.clone()).or_insert(0) += 1;
}

fn set_desktop_pool_session_metrics(smap: &mut SessionMap, s: &data::Session, id: &str) {
    match s.session_state.as_str() {
        constants::SESSION_CONNECTED
        | constants::SESSION_DISCONNECTED
        | constants::SESSION_PENDING => {}
        _ => {
            warn!(
                "skipping unknown state {} for session id {}",
                s.session_state, s.id
            );
        }
    };

    let ps = smap
        .entry(id.to_string())
        .or_insert_with(HashMap::<String, i64>::new);
    let lc_state = s.session_state.to_lowercase();

    *ps.entry(lc_state).or_insert(0) += 1;
}
