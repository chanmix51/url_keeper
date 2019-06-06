#[macro_use]
extern crate structopt;
extern crate serde;
extern crate url;

mod configuration;

use structopt::StructOpt;
use configuration::command_line::CommandLine;
use configuration::config_file::Config;

fn main() {
    let clo = CommandLine::from_args();
    let config_file = "config.yml";
    let config = match Config::from_file(config_file) {
        Ok(config) => config,
        Err(e) => {
            println!("Error. Could not read configuration from file {}.\n{:?}.", config_file, e);
            std::process::exit(1);
        }
    };
}
