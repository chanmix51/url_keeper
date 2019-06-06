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
    let config = match ConfigFile::from_file(clo.config_file.as_str()) {
        Ok(config) => config,
        Err(e) => {
            println!("Error. Could not read configuration from file '{}'.\n{:?}.", clo.config_file, e);
            std::process::exit(1);
        }
    };

    let logger = {
        let level: LevelFilter = match clo.verbosity {
            0 => LevelFilter::Error,
            1 => LevelFilter::Warn,
            2 => LevelFilter::Info,
            3 => LevelFilter::Debug,
            _ => LevelFilter::Trace,
        };

        TermLogger::new(level, Config::default()).unwrap()
    };

    println!("logger = {}", logger.level);
    info!("CLO = {:?}", clo);
}
