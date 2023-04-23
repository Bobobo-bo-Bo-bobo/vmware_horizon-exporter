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

pub const MSTATE_AGENT_CONFIG_ERROR: &str = "AGENT_CONFIG_ERROR";
pub const MSTATE_AGENT_DRAIN_MODE: &str = "AGENT_DRAIN_MODE";
pub const MSTATE_AGENT_DRAIN_UNTIL_RESTART: &str = "AGENT_DRAIN_UNTIL_RESTART";
pub const MSTATE_AGENT_ERROR_DISABLED: &str = "AGENT_ERROR_DISABLED";
pub const MSTATE_AGENT_ERROR_INVALID_IP: &str = "AGENT_ERROR_INVALID_IP";
pub const MSTATE_AGENT_ERROR_NEEDS_REBOOT: &str = "AGENT_ERROR_NEEDS_REBOOT";
pub const MSTATE_AGENT_ERROR_PROTOCOL_FAILURE: &str = "AGENT_ERROR_PROTOCOL_FAILURE";
pub const MSTATE_AGENT_ERROR_STARTUP_IN_PROGRESS: &str = "AGENT_ERROR_STARTUP_IN_PROGRESS";
pub const MSTATE_AGENT_ERROR_DOMAIN_FAILURE: &str = "AGENT_ERROR_DOMAIN_FAILURE";
pub const MSTATE_AGENT_UNREACHABLE: &str = "AGENT_UNREACHABLE";
pub const MSTATE_ALREADY_USED: &str = "ALREADY_USED";
pub const MSTATE_AVAILABLE: &str = "AVAILABLE";
pub const MSTATE_CONNECTED: &str = "CONNECTED";
pub const MSTATE_CUSTOMIZING: &str = "CUSTOMIZING";
pub const MSTATE_DELETING: &str = "DELETING";
pub const MSTATE_DISABLED: &str = "DISABLED";
pub const MSTATE_DISABLE_IN_PROGRESS: &str = "DISABLE_IN_PROGRESS";
pub const MSTATE_DISCONNECTED: &str = "DISCONNECTED";
pub const MSTATE_ERROR: &str = "ERROR";
pub const MSTATE_IN_PROGRESS: &str = "IN_PROGRESS";
pub const MSTATE_MAINTENANCE: &str = "MAINTENANCE";
pub const MSTATE_PROVISIONED: &str = "PROVISIONED";
pub const MSTATE_PROVISIONING: &str = "PROVISIONING";
pub const MSTATE_PROVISIONING_ERROR: &str = "PROVISIONING_ERROR";
pub const MSTATE_UNASSIGNED_USER_CONNECTED: &str = "UNASSIGNED_USER_CONNECTED";
pub const MSTATE_UNASSIGNED_USER_DISCONNECTED: &str = "UNASSIGNED_USER_DISCONNECTED";
pub const MSTATE_UNKNOWN: &str = "UNKNOWN";
pub const MSTATE_VALIDATING: &str = "VALIDATING";
pub const MSTATE_WAITING_FOR_AGENT: &str = "WAITING_FOR_AGENT";
pub const LC_MSTATE_AGENT_CONFIG_ERROR: &str = "agent_config_error";
pub const LC_MSTATE_AGENT_DRAIN_MODE: &str = "agent_drain_mode";
pub const LC_MSTATE_AGENT_DRAIN_UNTIL_RESTART: &str = "agent_drain_until_restart";
pub const LC_MSTATE_AGENT_ERROR_DISABLED: &str = "agent_error_disabled";
pub const LC_MSTATE_AGENT_ERROR_INVALID_IP: &str = "agent_error_invalid_ip";
pub const LC_MSTATE_AGENT_ERROR_NEEDS_REBOOT: &str = "agent_error_needs_reboot";
pub const LC_MSTATE_AGENT_ERROR_PROTOCOL_FAILURE: &str = "agent_error_protocol_failure";
pub const LC_MSTATE_AGENT_ERROR_STARTUP_IN_PROGRESS: &str = "agent_error_startup_in_progress";
pub const LC_MSTATE_AGENT_ERROR_DOMAIN_FAILURE: &str = "agent_error_domain_failure";
pub const LC_MSTATE_AGENT_UNREACHABLE: &str = "agent_unreachable";
pub const LC_MSTATE_ALREADY_USED: &str = "already_used";
pub const LC_MSTATE_AVAILABLE: &str = "available";
pub const LC_MSTATE_CONNECTED: &str = "connected";
pub const LC_MSTATE_CUSTOMIZING: &str = "customizing";
pub const LC_MSTATE_DELETING: &str = "deleting";
pub const LC_MSTATE_DISABLED: &str = "disabled";
pub const LC_MSTATE_DISABLE_IN_PROGRESS: &str = "disable_in_progress";
pub const LC_MSTATE_DISCONNECTED: &str = "disconnected";
pub const LC_MSTATE_ERROR: &str = "error";
pub const LC_MSTATE_IN_PROGRESS: &str = "in_progress";
pub const LC_MSTATE_MAINTENANCE: &str = "maintenance";
pub const LC_MSTATE_PROVISIONED: &str = "provisioned";
pub const LC_MSTATE_PROVISIONING: &str = "provisioning";
pub const LC_MSTATE_PROVISIONING_ERROR: &str = "provisioning_error";
pub const LC_MSTATE_UNASSIGNED_USER_CONNECTED: &str = "unassigned_user_connected";
pub const LC_MSTATE_UNASSIGNED_USER_DISCONNECTED: &str = "unassigned_user_disconnected";
pub const LC_MSTATE_UNKNOWN: &str = "unknown";
pub const LC_MSTATE_VALIDATING: &str = "validating";
pub const LC_MSTATE_WAITING_FOR_AGENT: &str = "waiting_for_agent";

pub const OS_LINUX_CENTOS: &str = "LINUX_CENTOS";
pub const OS_LINUX_OTHER: &str = "LINUX_OTHER";
pub const OS_LINUX_RHEL: &str = "LINUX_RHEL";
pub const OS_LINUX_SERVER_OTHER: &str = "LINUX_SERVER_OTHER";
pub const OS_LINUX_SUSE: &str = "LINUX_SUSE";
pub const OS_LINUX_UBUNTU: &str = "LINUX_UBUNTU";
pub const OS_UNKNOWN: &str = "UNKNOWN";
pub const OS_WINDOWS_10: &str = "WINDOWS_10";
pub const OS_WINDOWS_11: &str = "WINDOWS_11";
pub const OS_WINDOWS_7: &str = "WINDOWS_7";
pub const OS_WINDOWS_8: &str = "WINDOWS_8";
pub const OS_WINDOWS_SERVER_2003: &str = "WINDOWS_SERVER_2003";
pub const OS_WINDOWS_SERVER_2008: &str = "WINDOWS_SERVER_2008";
pub const OS_WINDOWS_SERVER_2008_R2: &str = "WINDOWS_SERVER_2008_R2";
pub const OS_WINDOWS_SERVER_2012: &str = "WINDOWS_SERVER_2012";
pub const OS_WINDOWS_SERVER_2012_R2: &str = "WINDOWS_SERVER_2012_R2";
pub const OS_WINDOWS_SERVER_2016_OR_ABOVE: &str = "WINDOWS_SERVER_2016_OR_ABOVE";
pub const OS_WINDOWS_VISTA: &str = "WINDOWS_VISTA";
pub const OS_WINDOWS_XP: &str = "WINDOWS_XP";
pub const LC_OS_LINUX_CENTOS: &str = "linux_centos";
pub const LC_OS_LINUX_OTHER: &str = "linux_other";
pub const LC_OS_LINUX_RHEL: &str = "linux_rhel";
pub const LC_OS_LINUX_SERVER_OTHER: &str = "linux_server_other";
pub const LC_OS_LINUX_SUSE: &str = "linux_suse";
pub const LC_OS_LINUX_UBUNTU: &str = "linux_ubuntu";
pub const LC_OS_UNKNOWN: &str = "unknown";
pub const LC_OS_WINDOWS_10: &str = "windows_10";
pub const LC_OS_WINDOWS_11: &str = "windows_11";
pub const LC_OS_WINDOWS_7: &str = "windows_7";
pub const LC_OS_WINDOWS_8: &str = "windows_8";
pub const LC_OS_WINDOWS_SERVER_2003: &str = "windows_server_2003";
pub const LC_OS_WINDOWS_SERVER_2008: &str = "windows_server_2008";
pub const LC_OS_WINDOWS_SERVER_2008_R2: &str = "windows_server_2008_r2";
pub const LC_OS_WINDOWS_SERVER_2012: &str = "windows_server_2012";
pub const LC_OS_WINDOWS_SERVER_2012_R2: &str = "windows_server_2012_r2";
pub const LC_OS_WINDOWS_SERVER_2016_OR_ABOVE: &str = "windows_server_2016_or_above";
pub const LC_OS_WINDOWS_VISTA: &str = "windows_vista";
pub const LC_OS_WINDOWS_XP: &str = "windows_xp";

pub const ARCH_BIT_64: &str = "BIT_64";
pub const ARCH_BIT_32: &str = "BIT_32";
pub const ARCH_UNKNOWN: &str = "UNKNOWN";
pub const LC_ARCH_BIT_64: &str = "bit_64";
pub const LC_ARCH_BIT_32: &str = "bit_32";
pub const LC_ARCH_UNKNOWN: &str = "unknown";

pub const REST_LOGIN: &str = "/rest/login";
pub const REST_LOGOUT: &str = "/rest/logout";
pub const REST_SESSIONS: &str = "/rest/inventory/v1/sessions";
pub const REST_DESKTOP_POOLS: &str = "/rest/inventory/v1/desktop-pools";
pub const REST_MACHINES: &str = "/rest/inventory/v1/machines";

pub const SESSIONS_NAME: &str = "horizon_sessions";
pub const SESSIONS_HELP: &str = "Horizon sessions";
pub const AGENT_VERSIONS_NAME: &str = "horizon_agent_version_info";
pub const AGENT_VERSIONS_HELP: &str = "Version of horizon agent";
pub const SESSION_PROTOCOLS_NAME: &str = "horizon_session_protocols";
pub const SESSION_PROTOCOLS_HELP: &str = "Horizon session protocols";
pub const SESSION_TYPES_NAME: &str = "horizon_session_types";
pub const SESSION_TYPES_HELP: &str = "Horizon session type";
pub const MACHINE_STATES_NAME: &str = "horizon_machine_states";
pub const MACHINE_STATES_HELP: &str = "State of horizon virtual machine";
pub const MACHINE_OS_NAME: &str = "horizon_machine_os_info";
pub const MACHINE_OS_HELP: &str = "Operating system on virtual machines";
pub const MACHINE_ARCH_NAME: &str = "horizon_machine_os_arch_info";
pub const MACHINE_ARCH_HELP: &str = "Architecture of operating system on virtual machine";

