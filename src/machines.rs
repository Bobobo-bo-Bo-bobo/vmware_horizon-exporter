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

// Map poolid -> machine state, count
type MachineStateMap = HashMap<String, HashMap<String, i64>>;

fn flush_machine_state_map(m: &mut MachineStateMap) {
    for (k1, v1) in m.iter_mut() {
        for (k2, v2) in v1.iter_mut() {
            debug!(
                "machines.rs:flush_machine_state_map: setting m[{}][{}] from {} to 0",
                k1, k2, *v2
            );
            *v2 = 0;
        }
    }
}

fn initialise_machine_map(m: &mut MachineStateMap, p: &str) {
    debug!(
        "machines.rs:initialise_machine_map: initialising MachineStateMap for {}",
        p
    );

    let pm = m
        .entry(p.to_string())
        .or_insert_with(HashMap::<String, i64>::new);
    pm.insert(constants::LC_MSTATE_AGENT_CONFIG_ERROR.to_string(), 0);
    pm.insert(constants::LC_MSTATE_AGENT_DRAIN_MODE.to_string(), 0);
    pm.insert(
        constants::LC_MSTATE_AGENT_DRAIN_UNTIL_RESTART.to_string(),
        0,
    );
    pm.insert(constants::LC_MSTATE_AGENT_ERROR_DISABLED.to_string(), 0);
    pm.insert(constants::LC_MSTATE_AGENT_ERROR_INVALID_IP.to_string(), 0);
    pm.insert(constants::LC_MSTATE_AGENT_ERROR_NEEDS_REBOOT.to_string(), 0);
    pm.insert(
        constants::LC_MSTATE_AGENT_ERROR_PROTOCOL_FAILURE.to_string(),
        0,
    );
    pm.insert(
        constants::LC_MSTATE_AGENT_ERROR_STARTUP_IN_PROGRESS.to_string(),
        0,
    );
    pm.insert(
        constants::LC_MSTATE_AGENT_ERROR_DOMAIN_FAILURE.to_string(),
        0,
    );
    pm.insert(constants::LC_MSTATE_AGENT_UNREACHABLE.to_string(), 0);
    pm.insert(constants::LC_MSTATE_ALREADY_USED.to_string(), 0);
    pm.insert(constants::LC_MSTATE_AVAILABLE.to_string(), 0);
    pm.insert(constants::LC_MSTATE_CONNECTED.to_string(), 0);
    pm.insert(constants::LC_MSTATE_CUSTOMIZING.to_string(), 0);
    pm.insert(constants::LC_MSTATE_DELETING.to_string(), 0);
    pm.insert(constants::LC_MSTATE_DISABLED.to_string(), 0);
    pm.insert(constants::LC_MSTATE_DISABLE_IN_PROGRESS.to_string(), 0);
    pm.insert(constants::LC_MSTATE_DISCONNECTED.to_string(), 0);
    pm.insert(constants::LC_MSTATE_ERROR.to_string(), 0);
    pm.insert(constants::LC_MSTATE_IN_PROGRESS.to_string(), 0);
    pm.insert(constants::LC_MSTATE_MAINTENANCE.to_string(), 0);
    pm.insert(constants::LC_MSTATE_PROVISIONED.to_string(), 0);
    pm.insert(constants::LC_MSTATE_PROVISIONING.to_string(), 0);
    pm.insert(constants::LC_MSTATE_PROVISIONING_ERROR.to_string(), 0);
    pm.insert(
        constants::LC_MSTATE_UNASSIGNED_USER_CONNECTED.to_string(),
        0,
    );
    pm.insert(
        constants::LC_MSTATE_UNASSIGNED_USER_DISCONNECTED.to_string(),
        0,
    );
    pm.insert(constants::LC_MSTATE_UNKNOWN.to_string(), 0);
    pm.insert(constants::LC_MSTATE_VALIDATING.to_string(), 0);
    pm.insert(constants::LC_MSTATE_WAITING_FOR_AGENT.to_string(), 0);
}

pub fn machine_metric_update(
    cfg: &configuration::Configuration,
    client: &mut reqwest::blocking::Client,
    token: &str,
) -> Result<(), Box<dyn Error>> {
    lazy_static! {
        static ref MSTATES: Mutex<MachineStateMap> = Mutex::new(HashMap::new());
    }
    let mut mstates = MSTATES.lock().unwrap();

    flush_machine_state_map(&mut mstates);

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
        if !mstates.contains_key(&dp.id) {
            initialise_machine_map(&mut mstates, &dp.id);
        }
    }

    debug!("machines.rs:machine_metric_update: getting list of current machines");
    let machines = horizon::get_machines(cfg, client, token)?;
    for m in machines.iter() {
        debug!(
            "machines.rs:session_metric_update: processing machine data - {:?}",
            m
        );
        if cfg.horizon_api.skip_pools_set.contains(&m.desktop_pool_id) {
            debug!(
                "machines.rs:machine_metric_update: desktop pool id {} is in skip_pools list",
                m.desktop_pool_id
            );
            continue;
        }

        if !cfg.horizon_api.only_pools_set.is_empty()
            && !cfg.horizon_api.only_pools_set.contains(&m.desktop_pool_id)
        {
            debug!("machines.rs:machine_metric_update: only_pools list is not empty and desktop pool id {} is not in only_pools list", m.desktop_pool_id);
            continue;
        }

        set_machine_state_metrics(&mut mstates, m);
    }

    prometheus_machine_states(&mstates);
    Ok(())
}

fn prometheus_machine_states(mmap: &MachineStateMap) {
    for (pool, mstate) in mmap.iter() {
        for (state, count) in mstate.iter() {
            exporter::MACHINE_STATES
                .with_label_values(&[pool, state])
                .set(*count);
        }
    }
}

fn set_machine_state_metrics(mmap: &mut MachineStateMap, m: &data::Machine) {
    match m.state.as_str() {
        constants::MSTATE_AGENT_CONFIG_ERROR
        | constants::MSTATE_AGENT_DRAIN_MODE
        | constants::MSTATE_AGENT_DRAIN_UNTIL_RESTART
        | constants::MSTATE_AGENT_ERROR_DISABLED
        | constants::MSTATE_AGENT_ERROR_INVALID_IP
        | constants::MSTATE_AGENT_ERROR_NEEDS_REBOOT
        | constants::MSTATE_AGENT_ERROR_PROTOCOL_FAILURE
        | constants::MSTATE_AGENT_ERROR_STARTUP_IN_PROGRESS
        | constants::MSTATE_AGENT_ERROR_DOMAIN_FAILURE
        | constants::MSTATE_AGENT_UNREACHABLE
        | constants::MSTATE_ALREADY_USED
        | constants::MSTATE_AVAILABLE
        | constants::MSTATE_CONNECTED
        | constants::MSTATE_CUSTOMIZING
        | constants::MSTATE_DELETING
        | constants::MSTATE_DISABLED
        | constants::MSTATE_DISABLE_IN_PROGRESS
        | constants::MSTATE_DISCONNECTED
        | constants::MSTATE_ERROR
        | constants::MSTATE_IN_PROGRESS
        | constants::MSTATE_MAINTENANCE
        | constants::MSTATE_PROVISIONED
        | constants::MSTATE_PROVISIONING
        | constants::MSTATE_PROVISIONING_ERROR
        | constants::MSTATE_UNASSIGNED_USER_CONNECTED
        | constants::MSTATE_UNASSIGNED_USER_DISCONNECTED
        | constants::MSTATE_UNKNOWN
        | constants::MSTATE_VALIDATING
        | constants::MSTATE_WAITING_FOR_AGENT => {}
        _ => {
            warn!("skipping unknown state {} for machine id {}", m.state, m.id);
        }
    };

    let ms = mmap
        .entry(m.desktop_pool_id.to_string())
        .or_insert_with(HashMap::new);
    let lc_state = m.state.to_lowercase();
    *ms.entry(lc_state).or_insert(0) += 1;
}

/*
*/
