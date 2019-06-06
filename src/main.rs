#[macro_use] extern crate structopt;
extern crate serde;
extern crate url;
#[macro_use] extern crate log;
extern crate simplelog;

mod configuration;

use structopt::StructOpt;
use configuration::command_line::CommandLine;
use configuration::config_file::ConfigFile;
use simplelog::{TermLogger, LevelFilter, Config};

fn main() {
    let clo = CommandLine::from_args();
    let _logger = {
        let level: LevelFilter = match clo.verbosity {
            0 => LevelFilter::Error,
            1 => LevelFilter::Warn,
            2 => LevelFilter::Info,
            3 => LevelFilter::Debug,
            _ => LevelFilter::Trace,
        };

        TermLogger::init(level, Config::default()).unwrap()
    };
    debug!("CLO = {:?}", clo);
    let config = match ConfigFile::from_file(clo.config_file.as_str()) {
        Ok(config) => config,
        Err(e) => {
            error!("Error. Could not read configuration from file '{}'.", clo.config_file);
            debug!("Error thrown is {:?}.", e);
            std::process::exit(1);
        }
    };
}
