use serde::Deserialize;
use simple_error::bail;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs;

#[derive(Clone, Debug, Deserialize)]
pub struct Configuration {
    pub horizon_api: HorizonAPIConfig,
}

#[derive(Clone, Debug, Deserialize)]
pub struct HorizonAPIConfig {
    pub url: String,
    pub user: String,
    pub domain: String,
    pub password: String,
    pub insecure_ssl: Option<bool>,
    pub ca_file: Option<String>,
    pub timeout: Option<u64>,
    pub only_pools: Option<Vec<String>>,
    pub skip_pools: Option<Vec<String>>,
    pub pool_uuid_map: Option<HashMap<String, String>>,
    #[serde(skip)]
    pub only_pools_set: HashSet<String>,
    #[serde(skip)]
    pub skip_pools_set: HashSet<String>,
}

impl HorizonAPIConfig {
    pub fn user_defined_pool_uuid_resolve(self, uuid: &str) -> String {
        match self.pool_uuid_map {
            // Some(m) => uuid.to_string(),
            Some(m) => match m.get(uuid) {
                Some(v) => v.to_string(),
                None => uuid.to_string(),
            },
            None => uuid.to_string(),
        }
    }
}

pub fn parse_config_file(f: &str) -> Result<Configuration, Box<dyn Error>> {
    let raw = fs::read_to_string(f)?;
    let mut config: Configuration = serde_yaml::from_str(raw.as_str())?;

    config.horizon_api.only_pools_set = HashSet::new();
    if let Some(v) = &config.horizon_api.only_pools {
        for o in v {
            config.horizon_api.only_pools_set.insert(o.clone());
        }
    }

    config.horizon_api.skip_pools_set = HashSet::new();
    if let Some(v) = &config.horizon_api.skip_pools {
        for s in v {
            config.horizon_api.skip_pools_set.insert(s.clone());
        }
    }

    validate_config(&config)?;

    Ok(config)
}

fn validate_config(cfg: &Configuration) -> Result<(), Box<dyn Error>> {
    if cfg.horizon_api.url.is_empty() {
        bail!("Missing URL");
    }

    if cfg.horizon_api.user.is_empty() {
        bail!("Missing user for authentication");
    }

    if cfg.horizon_api.password.is_empty() {
        bail!("Missing password for authentication");
    }

    if cfg.horizon_api.domain.is_empty() {
        bail!("Missing login domain for authentication");
    }

    for op in cfg.horizon_api.only_pools_set.iter() {
        if cfg.horizon_api.skip_pools_set.contains(op) {
            bail!("pool {} is in only_pools and skip_pools", op);
        }
    }
    Ok(())
}
