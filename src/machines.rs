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

// Map poolid -> os, count
type MachineOSMap = HashMap<String, HashMap<String, i64>>;

// Map poolid -> arch, count
type MachineArchMap = HashMap<String, HashMap<String, i64>>;

fn flush_machine_arch_map(m: &mut MachineOSMap) {
    for (k1, v1) in m.iter_mut() {
        for (k2, v2) in v1.iter_mut() {
            debug!(
                "machines.rs:flush_machine_arch_map: setting m[{}][{}] from {} to 0",
                k1, k2, *v2
            );
            *v2 = 0;
        }
    }
}

fn flush_machine_os_map(m: &mut MachineOSMap) {
    for (k1, v1) in m.iter_mut() {
        for (k2, v2) in v1.iter_mut() {
            debug!(
                "machines.rs:flush_machine_os_map: setting m[{}][{}] from {} to 0",
                k1, k2, *v2
            );
            *v2 = 0;
        }
    }
}

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

fn initialise_machine_arch_map(m: &mut MachineOSMap, p: &str) {
    debug!(
        "machines.rs:initialise_machine_arch_map: initialising MachineArchMap for {}",
        p
    );

    let pm = m
        .entry(p.to_string())
        .or_insert_with(HashMap::<String, i64>::new);
    pm.insert(constants::LC_ARCH_BIT_64.to_string(), 0);
    pm.insert(constants::LC_ARCH_BIT_32.to_string(), 0);
    pm.insert(constants::LC_ARCH_UNKNOWN.to_string(), 0);
}

fn initialise_machine_os_map(m: &mut MachineOSMap, p: &str) {
    debug!(
        "machines.rs:initialise_machine_os_map: initialising MachineOSMap for {}",
        p
    );

    let pm = m
        .entry(p.to_string())
        .or_insert_with(HashMap::<String, i64>::new);

    pm.insert(constants::LC_OS_LINUX_CENTOS.to_string(), 0);
    pm.insert(constants::LC_OS_LINUX_OTHER.to_string(), 0);
    pm.insert(constants::LC_OS_LINUX_RHEL.to_string(), 0);
    pm.insert(constants::LC_OS_LINUX_SERVER_OTHER.to_string(), 0);
    pm.insert(constants::LC_OS_LINUX_SUSE.to_string(), 0);
    pm.insert(constants::LC_OS_LINUX_UBUNTU.to_string(), 0);
    pm.insert(constants::LC_OS_UNKNOWN.to_string(), 0);
    pm.insert(constants::LC_OS_WINDOWS_10.to_string(), 0);
    pm.insert(constants::LC_OS_WINDOWS_11.to_string(), 0);
    pm.insert(constants::LC_OS_WINDOWS_7.to_string(), 0);
    pm.insert(constants::LC_OS_WINDOWS_8.to_string(), 0);
    pm.insert(constants::LC_OS_WINDOWS_SERVER_2003.to_string(), 0);
    pm.insert(constants::LC_OS_WINDOWS_SERVER_2008.to_string(), 0);
    pm.insert(constants::LC_OS_WINDOWS_SERVER_2008_R2.to_string(), 0);
    pm.insert(constants::LC_OS_WINDOWS_SERVER_2012.to_string(), 0);
    pm.insert(constants::LC_OS_WINDOWS_SERVER_2012_R2.to_string(), 0);
    pm.insert(constants::LC_OS_WINDOWS_SERVER_2016_OR_ABOVE.to_string(), 0);
    pm.insert(constants::LC_OS_WINDOWS_VISTA.to_string(), 0);
    pm.insert(constants::LC_OS_WINDOWS_XP.to_string(), 0);
}

fn initialise_machine_state_map(m: &mut MachineStateMap, p: &str) {
    debug!(
        "machines.rs:initialise_machine_state_map: initialising MachineStateMap for {}",
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
        static ref OS: Mutex<MachineOSMap> = Mutex::new(HashMap::new());
        static ref ARCH: Mutex<MachineArchMap> = Mutex::new(HashMap::new());
    }
    let mut mstates = MSTATES.lock().unwrap();
    let mut os_map = OS.lock().unwrap();
    let mut arch_map = ARCH.lock().unwrap();

    flush_machine_state_map(&mut mstates);
    flush_machine_os_map(&mut os_map);
    flush_machine_arch_map(&mut arch_map);

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
            initialise_machine_state_map(&mut mstates, &dp.id);
            initialise_machine_os_map(&mut os_map, &dp.id);
            initialise_machine_arch_map(&mut arch_map, &dp.id);
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
        set_machine_os_metrics(&mut os_map, m);
        set_machine_arch_metrics(&mut arch_map, m);
    }

    prometheus_machine_states(&mstates);
    prometheus_machine_os(&os_map);
    prometheus_machine_arch(&arch_map);
    Ok(())
}

fn prometheus_machine_arch(amap: &MachineArchMap) {
    for (pool, archname) in amap.iter() {
        for (arch, count) in archname.iter() {
            exporter::MACHINE_ARCH
                .with_label_values(&[pool, arch])
                .set(*count);
        }
    }
}

fn prometheus_machine_os(omap: &MachineOSMap) {
    for (pool, osname) in omap.iter() {
        for (os, count) in osname.iter() {
            exporter::MACHINE_OS
                .with_label_values(&[pool, os])
                .set(*count);
        }
    }
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

fn set_machine_arch_metrics(amap: &mut MachineArchMap, m: &data::Machine) {
    if let Some(arch) = &m.operating_system_architecture {
        match arch.as_str() {
            constants::ARCH_BIT_64 | constants::ARCH_BIT_32 | constants::ARCH_UNKNOWN => {},
            _ => {
                warn!("skipping unknown architecture {} for machine id {}", arch, m.id);
            }
        };
        let am = amap.entry(m.desktop_pool_id.to_string()).or_insert_with(HashMap::new);
        let lc_arch = arch.to_lowercase();
        *am.entry(lc_arch).or_insert(0) += 1;
    }
}

fn set_machine_os_metrics(omap: &mut MachineOSMap, m: &data::Machine) {
    if let Some(os) = &m.operating_system {
        match os.as_str() {
            constants::OS_LINUX_CENTOS
            | constants::OS_LINUX_OTHER
            | constants::OS_LINUX_RHEL
            | constants::OS_LINUX_SERVER_OTHER
            | constants::OS_LINUX_SUSE
            | constants::OS_LINUX_UBUNTU
            | constants::OS_UNKNOWN
            | constants::OS_WINDOWS_10
            | constants::OS_WINDOWS_11
            | constants::OS_WINDOWS_7
            | constants::OS_WINDOWS_8
            | constants::OS_WINDOWS_SERVER_2003
            | constants::OS_WINDOWS_SERVER_2008
            | constants::OS_WINDOWS_SERVER_2008_R2
            | constants::OS_WINDOWS_SERVER_2012
            | constants::OS_WINDOWS_SERVER_2012_R2
            | constants::OS_WINDOWS_SERVER_2016_OR_ABOVE
            | constants::OS_WINDOWS_VISTA
            | constants::OS_WINDOWS_XP => {}
            _ => {
                warn!(
                    "skipping unknown operating system {} for machine id {}",
                    os, m.id
                );
            }
        };

        let om = omap
            .entry(m.desktop_pool_id.to_string())
            .or_insert_with(HashMap::new);
        let lc_os = os.to_lowercase();
        *om.entry(lc_os).or_insert(0) += 1;
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
