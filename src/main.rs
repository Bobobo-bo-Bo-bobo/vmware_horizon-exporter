use getopts::Options;
use log::{debug, error};
use std::{env, process};

mod configuration;
mod constants;
mod data;
mod exporter;
mod globals;
mod horizon;
mod http;
mod machines;
mod sessions;
mod usage;

fn main() {
    let argv: Vec<String> = env::args().collect();
    let mut options = Options::new();
    let mut log_level = log::LevelFilter::Info;

    options.optflag("D", "debug", "Enable debug output");
    options.optflag("V", "version", "Show version information");
    options.optflag("Q", "quiet", "Only log warning and error messages");
    options.optopt("c", "config", "Path to configuration file", "<config_file>");
    options.optflag("h", "help", "Show help text");
    options.optopt(
        "l",
        "listen",
        "Listen on <addr> for metric scrapes",
        "<addr>",
    );

    let opts = match options.parse(&argv[1..]) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Error: Unable to parse command line options: {}", e);
            println!();
            usage::show_usage();
            process::exit(1);
        }
    };

    if opts.opt_present("D") {
        log_level = log::LevelFilter::Debug;
    }

    if opts.opt_present("V") {
        usage::show_version();
        process::exit(0);
    }

    if opts.opt_present("Q") {
        log_level = log::LevelFilter::Warn;
    }

    if opts.opt_present("h") {
        usage::show_usage();
        process::exit(0);
    }

    let config_file = match opts.opt_str("c") {
        Some(v) => v,
        None => {
            eprintln!("Error: Missing configuration file");
            println!();
            usage::show_usage();
            process::exit(1);
        }
    };

    let listen_address = opts
        .opt_str("l")
        .unwrap_or_else(|| constants::DEFAULT_LISTEN_ADDR.to_string());

    match logging_init(log_level) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Error: Can't initialise logging: {}", e);
            process::exit(1);
        }
    };

    let config = match configuration::parse_config_file(&config_file) {
        Ok(v) => v,
        Err(e) => {
            error!("can't parse configuration file: {}", e);
            process::exit(1);
        }
    };

    debug!(
        "main.rs:main: parsed configuration fomr {} - {:?}",
        config_file, config
    );

    exporter::register_metrics();

    if let Err(e) = http::server(config, &listen_address) {
        error!("can't start HTTP server: {}", e);
        process::exit(1);
    };
}

fn logging_init(l: log::LevelFilter) -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .format(|logout, logmsg, logrecord| {
            logout.finish(format_args!(
                "{:<6}: {} {}",
                logrecord.level(),
                chrono::Local::now().format("%Y-%m-%dT%H:%M:%S%z"),
                logmsg
            ))
        })
        .level(l)
        .chain(std::io::stdout())
        .apply()?;
    Ok(())
}
