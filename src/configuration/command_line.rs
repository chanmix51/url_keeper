use crate::structopt::StructOpt;
use crate::url::Url;

#[derive(Debug, StructOpt)]
#[structopt(name = "url_keeper", about = "Remember urls for you.", author = "Grégoire HUBERT <hubert.greg@gmail.com>")]
pub struct CommandLine {
    #[structopt(short = "c", long = "config-file", default_value = "config.yml")]
    pub config_file: String,
    #[structopt(short = "v", parse(from_occurrences), help = "Set verbosity level (can be specified multiple times).")]
    pub verbosity: u32,
    #[structopt(subcommand)]
    pub command: SubCommand,
}

#[derive(Debug, StructOpt)]
pub enum SubCommand {
    #[structopt(name = "add")]
    Add {
        #[structopt(name = "url")]
        url: Url,
        #[structopt(short = "t", long = "tags")]
        tags: Vec<String>,
    }
}
