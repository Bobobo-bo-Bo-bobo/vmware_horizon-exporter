use crate::configuration;
use crate::constants;
use crate::globals;
use crate::horizon;
use crate::http;
use crate::machines;
use crate::sessions;

use lazy_static::lazy_static;
use log::{debug, error, warn};
use prometheus::{IntGaugeVec, Opts, Registry, TextEncoder};

lazy_static! {
    pub static ref REGISTRY: Registry = Registry::new();
    pub static ref SESSIONS: IntGaugeVec = IntGaugeVec::new(
        Opts::new(constants::SESSIONS_NAME, constants::SESSIONS_HELP),
        &["pool", "state"]
    )
    .unwrap();
    pub static ref AGENT_VERSIONS: IntGaugeVec = IntGaugeVec::new(
        Opts::new(
            constants::AGENT_VERSIONS_NAME,
            constants::AGENT_VERSIONS_HELP
        ),
        &["version"]
    )
    .unwrap();
    pub static ref SESSION_PROTOCOLS: IntGaugeVec = IntGaugeVec::new(
        Opts::new(
            constants::SESSION_PROTOCOLS_NAME,
            constants::SESSION_PROTOCOLS_HELP
        ),
        &["pool", "protocol"]
    )
    .unwrap();
    pub static ref SESSION_TYPES: IntGaugeVec = IntGaugeVec::new(
        Opts::new(constants::SESSION_TYPES_NAME, constants::SESSION_TYPES_HELP),
        &["pool", "type"]
    )
    .unwrap();
    pub static ref MACHINE_STATES: IntGaugeVec = IntGaugeVec::new(
        Opts::new(
            constants::MACHINE_STATES_NAME,
            constants::MACHINE_STATES_HELP
        ),
        &["pool", "state"],
    )
    .unwrap();
}

pub fn register_metrics() {
    REGISTRY.register(Box::new(SESSIONS.clone())).unwrap();
    REGISTRY.register(Box::new(AGENT_VERSIONS.clone())).unwrap();
    REGISTRY
        .register(Box::new(SESSION_PROTOCOLS.clone()))
        .unwrap();
    REGISTRY.register(Box::new(SESSION_TYPES.clone())).unwrap();
    REGISTRY.register(Box::new(MACHINE_STATES.clone())).unwrap();
}

fn metric_update(cfg: &configuration::Configuration, client: &mut reqwest::blocking::Client) {
    debug!("exporter.rs:metric_update: login to horizon");
    let tokens = match horizon::login(cfg, client) {
        Ok(v) => v,
        Err(e) => {
            error!("horizon login failed: {}", e);
            return;
        }
    };

    // fetch pool data only once
    {
        let mut desktop_pools = globals::DESKTOP_POOLS.lock().unwrap();
        debug!("exporter.rs:metric_update: getting list of desktop pools");
        *desktop_pools = match horizon::get_desktop_pools(cfg, client, &tokens.access_token) {
            Ok(v) => v,
            Err(e) => {
                error!("can't get list of desktop pools: {}", e);
                return;
            }
        };
    }

    if let Err(e) = sessions::session_metric_update(cfg, client, &tokens.access_token) {
        error!("session metric update failed: {}", e);
    }

    if let Err(e) = machines::machine_metric_update(cfg, client, &tokens.access_token) {
        error!("machine metric update failed: {}", e);
    }

    debug!("exporter.rs:metric_update: logout from horizon");
    if let Err(e) = horizon::logout(cfg, client, &tokens.refresh_token) {
        warn!("horizon logout failed: {}", e);
    }
}

pub fn fetch(cfg: &configuration::Configuration) -> String {
    let insecure_ssl = cfg.horizon_api.insecure_ssl.unwrap_or_default();
    let ca_file = match &cfg.horizon_api.ca_file {
        Some(v) => v,
        None => "",
    };
    let timeout = cfg
        .horizon_api
        .timeout
        .unwrap_or(constants::DEFAULT_TIMEOUT);

    let mut http_client = match http::build_client(insecure_ssl, ca_file, timeout) {
        Ok(v) => v,
        Err(e) => {
            error!("can't create HTTP client structure: {}", e);
            return String::new();
        }
    };

    metric_update(cfg, &mut http_client);

    let encoder = TextEncoder::new();
    let mut buffer = String::new();

    if let Err(e) = encoder.encode_utf8(&REGISTRY.gather(), &mut buffer) {
        error!("unable to encode collected metrics as UTF-8 string: {}", e);
    }

    buffer
}
