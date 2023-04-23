use serde::{Deserialize, Serialize};

#[derive(Deserialize, Clone, Debug)]
pub struct ErrorResponse {
    pub status: String,
    pub timestamp: i64,
    pub errors: Vec<ErrorMessage>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct ErrorMessage {
    pub error_key: String,
    pub error_message: String,
}

#[derive(Serialize, Clone, Debug)]
pub struct LoginRequest {
    pub domain: String,
    pub password: String,
    pub username: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct LoginResponse {
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(Serialize, Clone, Debug)]
pub struct LogoutRequest {
    pub refresh_token: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Session {
    pub id: String,
    pub user_id: String,
    pub desktop_pool_id: Option<String>,
    pub agent_version: String,
    pub session_type: String,
    pub session_state: String,
    pub session_protocol: Option<String>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct DesktopPool {
    pub id: String,
    pub enabled: bool,
}
