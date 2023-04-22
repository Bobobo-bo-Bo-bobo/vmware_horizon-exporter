use crate::constants;

pub fn show_version() {
    println!(
        "{} version {}
Copyright (C) 2022-2023 by Andreas Maus <maus@ypbind.de>
This program comes with ABSOLUTELY NO WARRANTY.
    
{} is distributed under the Terms of the GNU General
Public License Version 3. (http://www.gnu.org/copyleft/gpl.html)
",
        constants::NAME,
        constants::VERSION,
        constants::NAME
    );
}

pub fn show_usage() {
    show_version();
    println!("Usage: {} [-D|--debug] [-V|--version] [-Q|--quiet] -c <cfg>|--config=<cfg> [-h|--help] [-l <addr>|--listen=<addr>]

    -D              Enable debug output
    --debug

    -V              Show version information
    --version

    -Q              Only log warning and error messages
    --quiet

    -c <cfg>        Path to configuration file
    --config=<cfg>

    -h              Show help text
    --help

    -l <addr>       Listen on <addr> for metric scrapes
    --listen=<addr> Default: {}
", constants::NAME, constants::DEFAULT_LISTEN_ADDR);
}
