use crate::configuration;
use crate::constants;
use crate::data;
use crate::exporter;
use crate::horizon;

use lazy_static::lazy_static;
use log::{debug, warn};
use std::collections::HashMap;
use std::error::Error;
use std::sync::Mutex;

// Map pool id -> state, count
type SessionMap = HashMap<String, HashMap<String, i64>>;

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
    }
    let mut pool_sessions = POOL_SESSIONS.lock().unwrap();

    // flush existing counters to prevent reporting of stale data
    flush_session_map(&mut pool_sessions);

    debug!("sessions.rs:session_metric_update: getting list of desktop pools");
    let dsktp_pools = horizon::get_desktop_pools(cfg, client, token)?;

    for dp in dsktp_pools {
        if cfg.horizon_api.skip_pools_set.contains(&dp.id) {
            continue;
        }
        if !cfg.horizon_api.only_pools_set.is_empty() {
            if !cfg.horizon_api.only_pools_set.contains(&dp.id) {
                continue;
            }
        }
        if !pool_sessions.contains_key(&dp.id) {
            initialise_session_map(&mut pool_sessions, &dp.id);
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

            if !cfg.horizon_api.only_pools_set.is_empty() {
                if !cfg.horizon_api.only_pools_set.contains(dp_id) {
                    debug!("sessions.rs:session_metric_update: only_pools list is not empty and desktop pool id {} is not in only_pools list", dp_id);
                    continue;
                }
            }

            set_desktop_pool_session_metrics(&mut pool_sessions, s, dp_id);
        } else {
            warn!("BUG: session id {} is not a desktop session", s.id);
        }
    }

    for (pool, scount) in pool_sessions.iter() {
        for (state, count) in scount.iter() {
            exporter::SESSIONS
                .with_label_values(&[&pool, &state])
                .set(*count);
        }
    }
    Ok(())
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