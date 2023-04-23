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

pub const PROTOCOL_PCOIP: &str = "PCOIP";
pub const PROTOCOL_RDP: &str = "RDP";
pub const PROTOCOL_BLAST: &str = "BLAST";
pub const PROTOCOL_CONSOLE: &str = "CONSOLE";
pub const PROTOCOL_UNKNOWN: &str = "UNKNOWN";
pub const LC_PROTOCOL_PCOIP: &str = "pcoip";
pub const LC_PROTOCOL_RDP: &str = "rdp";
pub const LC_PROTOCOL_BLAST: &str = "blast";
pub const LC_PROTOCOL_CONSOLE: &str = "console";
pub const LC_PROTOCOL_UNKNOWN: &str = "unknown";

pub const TYPE_APPLICATION: &str = "APPLICATION";
pub const TYPE_DESKTOP: &str = "DESKTOP";
pub const LC_TYPE_APPLICATION: &str = "application";
pub const LC_TYPE_DESKTOP: &str = "desktop";

pub const REST_LOGIN: &str = "/rest/login";
pub const REST_LOGOUT: &str = "/rest/logout";
pub const REST_SESSIONS: &str = "/rest/inventory/v1/sessions";
pub const REST_DESKTOP_POOLS: &str = "/rest/inventory/v1/desktop-pools";

pub const SESSIONS_NAME: &str = "horizon_sessions";
pub const SESSIONS_HELP: &str = "Horizon sessions";
pub const AGENT_VERSIONS_NAME: &str = "horizon_agent_version_info";
pub const AGENT_VERSIONS_HELP: &str = "Version of horizon agent";
pub const SESSION_PROTOCOLS_NAME: &str = "horizon_session_protocols";
pub const SESSION_PROTOCOLS_HELP: &str = "Horizon session protocols";
pub const SESSION_TYPES_NAME: &str = "horizon_session_types";
pub const SESSION_TYPES_HELP: &str = "Horizon session type";
