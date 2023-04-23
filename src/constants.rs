pub const NAME: &str = env!("CARGO_PKG_NAME");
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
const REPO_URL: &str = "https://ypbind.de/cgit/vmware_horizon-exporter/";

pub fn generate_default_user_agent() -> String {
    format!("{}/{} ({})", NAME, VERSION, REPO_URL)
}

pub const ROOT_HTML: &str = "<html>\n<head><title>VMWare Horizon exporter</title></head>\n<body>\n<h1>VMWare Horizon exporter</h1>\n<p><a href=\"/metrics\">Metrics</a></p>\n</body>\n</html>\n";
pub const METRICS_PATH: &str = "/metrics";

pub const DEFAULT_LISTEN_ADDR: &str = "localhost:9133";
pub const DEFAULT_TIMEOUT: u64 = 60;

pub const REPLY_METHOD_NOT_ALLOWED: &str = "Method not allowed";
pub const REPLY_NOT_FOUND: &str = "Not found";

pub const SESSION_CONNECTED: &str = "CONNECTED";
pub const SESSION_DISCONNECTED: &str = "DISCONNECTED";
pub const SESSION_PENDING: &str = "PENDING";
pub const LC_SESSION_CONNECTED: &str = "connected";
pub const LC_SESSION_DISCONNECTED: &str = "disconnected";
pub const LC_SESSION_PENDING: &str = "pending";

pub const REST_LOGIN: &str = "/rest/login";
pub const REST_LOGOUT: &str = "/rest/logout";
pub const REST_SESSIONS: &str = "/rest/inventory/v1/sessions";
pub const REST_DESKTOP_POOLS: &str = "/rest/inventory/v1/desktop-pools";

pub const SESSIONS_NAME: &str = "horizon_sessions";
pub const SESSIONS_HELP: &str = "Horizon sessions";
pub const AGENT_VERSIONS_NAME: &str = "horizon_agent_version_info";
pub const AGENT_VERSIONS_HELP: &str = "Version of horizon agent";
