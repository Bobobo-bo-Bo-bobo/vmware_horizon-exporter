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
        bail!(
            "login failed, received {} instead of 200: {}",
            st,
            format_error_message(&lg_str)
        );
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
        bail!(
            "logout failed, received {} instead of 200: {}",
            st,
            format_error_message(&lg_str)
        );
    }

    Ok(())
}

pub fn get_sessions(
    cfg: &configuration::Configuration,
    cli: &mut reqwest::blocking::Client,
    token: &str,
) -> Result<Vec<data::Session>, Box<dyn Error>> {
    debug!(
        "horizon.rs:get_sessions: requesting session list from {}{}",
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
        bail!(
            "logout failed, received {} instead of 200: {}",
            st,
            format_error_message(&sess)
        );
    }

    let slist: Vec<data::Session> = serde_json::from_str(sess.as_str())?;
    debug!("horizon.rs:get_sessions: {} sessions in list", slist.len());

    Ok(slist)
}

pub fn get_session_for_machine_id(
    cfg: &configuration::Configuration,
    cli: &mut reqwest::blocking::Client,
    token: &str,
    machine_id: &str,
) -> Result<Option<data::Session>, Box<dyn Error>> {
    let mut filter = data::FilterPayload {
        filter_type: "And".to_string(),
        filters: Vec::new(),
    };
    let mid_filter = data::FilterRequest {
        comparison: "Equals".to_string(),
        name: "machine_id".to_string(),
        value: machine_id.to_string(),
    };
    filter.filters.push(mid_filter);
    let filter_str = serde_json::to_string(&filter)?;
    let encoded_filter = urlencoding::encode(&filter_str);

    debug!(
        "horizon.rs:get_session_for_machine_id: requesting session list from {}{}?filter={}",
        cfg.horizon_api.url,
        constants::REST_SESSIONS,
        encoded_filter,
    );

    let (st, sess) = http::get(
        cli,
        &format!(
            "{}{}?filter={}",
            cfg.horizon_api.url,
            constants::REST_SESSIONS,
            encoded_filter
        ),
        token,
    )?;
    debug!(
        "horizon.rs:get_session_for_machine_id: received HTTP status={}",
        st
    );

    if st != reqwest::StatusCode::OK && st != reqwest::StatusCode::NOT_FOUND {
        bail!(
            "logout failed, received {} instead of 200 or 404: {}",
            st,
            format_error_message(&sess)
        );
    }

    if st == reqwest::StatusCode::NOT_FOUND {
        return Ok(None);
    }

    let slist: Vec<data::Session> = serde_json::from_str(sess.as_str())?;
    debug!(
        "horizon.rs:get_session_for_machine_id: {} sessions in list",
        slist.len()
    );
    if slist.is_empty() {
        return Ok(None);
    }
    Ok(Some(slist[0].clone()))
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
        bail!(
            "logout failed, received {} instead of 200: {}",
            st,
            format_error_message(&dpl)
        );
    }

    let dplist: Vec<data::DesktopPool> = serde_json::from_str(dpl.as_str())?;
    debug!(
        "horizon.rs:get_desktop_pools: {} pools in list",
        dplist.len()
    );

    Ok(dplist)
}

pub fn get_machines(
    cfg: &configuration::Configuration,
    cli: &mut reqwest::blocking::Client,
    token: &str,
) -> Result<Vec<data::Machine>, Box<dyn Error>> {
    debug!(
        "horizon.rs:get_machines: requesting machine list from {}{}",
        cfg.horizon_api.url,
        constants::REST_MACHINES
    );

    let (st, mach) = http::get(
        cli,
        &format!("{}{}", cfg.horizon_api.url, constants::REST_MACHINES),
        token,
    )?;
    debug!("horizon.rs:get_machines: received HTTP status={}", st);

    if st != reqwest::StatusCode::OK {
        bail!(
            "logout failed, received {} instead of 200 - {}",
            st,
            format_error_message(&mach)
        );
    }

    let mlist: Vec<data::Machine> = serde_json::from_str(mach.as_str())?;
    debug!("horizon.rs:get_machines: {} machines in list", mlist.len());

    Ok(mlist)
}

fn format_error_message(e: &str) -> String {
    let err: data::ErrorResponse = match serde_json::from_str(e) {
        Ok(v) => v,
        Err(e) => {
            return format!("Can't decode response as JSON - {}", e);
        }
    };
    let mut msg_list: Vec<String> = Vec::new();

    for em in err.errors.iter() {
        msg_list.push(format!("{}: {}", em.error_key, em.error_message));
    }

    msg_list.join(", ")
}
