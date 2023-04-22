use crate::configuration;
use crate::constants;
use crate::data;
use crate::http;

use log::debug;
use simple_error::bail;
use std::error::Error;

pub fn login(
    cfg: &configuration::Configuration,
    cli: &mut reqwest::blocking::Client,
) -> Result<data::LoginResponse, Box<dyn Error>> {
    let lgi = data::LoginRequest {
        domain: cfg.horizon_api.domain.clone(),
        password: cfg.horizon_api.password.clone(),
        username: cfg.horizon_api.user.clone(),
    };

    let payload = serde_json::to_string(&lgi)?;

    debug!(
        "horizon.rs:login: sending login data to {}{} - {:?}",
        cfg.horizon_api.url,
        constants::REST_LOGIN,
        lgi
    );

    let (st, lg_str) = http::post(
        cli,
        &format!("{}{}", cfg.horizon_api.url, constants::REST_LOGIN),
        &payload,
        None,
    )?;

    debug!(
        "horizon.rs:login: received response HTTP status={} - {:?}",
        st, lg_str
    );

    if st != reqwest::StatusCode::OK {
        // TODO: Decode JSON error string
        bail!("login failed, received {} instead of 200", st);
    }

    let result = serde_json::from_str(lg_str.as_str())?;

    Ok(result)
}

pub fn logout(
    cfg: &configuration::Configuration,
    cli: &mut reqwest::blocking::Client,
    token: &str,
) -> Result<(), Box<dyn Error>> {
    let lgo = data::LogoutRequest {
        refresh_token: token.to_string(),
    };

    let payload = serde_json::to_string(&lgo)?;

    debug!(
        "horizon.rs:logout: sending login data to {}{} - {:?}",
        cfg.horizon_api.url,
        constants::REST_LOGOUT,
        lgo
    );

    let (st, lg_str) = http::post(
        cli,
        &format!("{}{}", cfg.horizon_api.url, constants::REST_LOGOUT),
        &payload,
        None,
    )?;

    debug!(
        "horizon.rs:logout: received response HTTP status={} - {:?}",
        st, lg_str
    );

    if st != reqwest::StatusCode::OK {
        // TODO: Decode JSON error string
        bail!("logout failed, received {} instead of 200", st);
    }

    Ok(())
}

pub fn get_sessions(
    cfg: &configuration::Configuration,
    cli: &mut reqwest::blocking::Client,
    token: &str,
) -> Result<Vec<data::Session>, Box<dyn Error>> {
    debug!(
        "horizon.rs:get_sesions: requesting session list from {}{}",
        cfg.horizon_api.url,
        constants::REST_SESSIONS
    );

    let (st, sess) = http::get(
        cli,
        &format!("{}{}", cfg.horizon_api.url, constants::REST_SESSIONS),
        token,
    )?;
    debug!("horizon.rs:get_sessions: received HTTP status={}", st);

    if st != reqwest::StatusCode::OK {
        // TODO: Decode JSON error string
        bail!("logout failed, received {} instead of 200", st);
    }

    let slist: Vec<data::Session> = serde_json::from_str(sess.as_str())?;
    debug!("horizon.rs:get_sessions: {} sessions in list", slist.len());

    Ok(slist)
}

pub fn get_desktop_pools(
    cfg: &configuration::Configuration,
    cli: &mut reqwest::blocking::Client,
    token: &str,
) -> Result<Vec<data::DesktopPool>, Box<dyn Error>> {
    debug!(
        "horizon.rs:get_desktop_pools: requesting desktop pool list from {}{}",
        cfg.horizon_api.url,
        constants::REST_DESKTOP_POOLS
    );

    let (st, dpl) = http::get(
        cli,
        &format!("{}{}", cfg.horizon_api.url, constants::REST_DESKTOP_POOLS),
        token,
    )?;
    debug!("horizon.rs:get_desktop_pools: received HTTP status={}", st);

    if st != reqwest::StatusCode::OK {
        // TODO: Decode JSON error string
        bail!("logout failed, received {} instead of 200", st);
    }

    let dplist: Vec<data::DesktopPool> = serde_json::from_str(dpl.as_str())?;
    debug!(
        "horizon.rs:get_desktop_pools: {} pools in list",
        dplist.len()
    );

    Ok(dplist)
}
