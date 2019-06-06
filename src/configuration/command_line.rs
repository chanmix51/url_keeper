use crate::structopt::StructOpt;
use crate::url::Url;

#[derive(Debug, StructOpt)]
#[structopt(name = "url_keeper", about = "Remember urls for you.", author = "Grégoire HUBERT <hubert.greg@gmail.com>.\nLicence CC-BY-SA 2019 https://creativecommons.org")]
pub enum CommandLine {
    #[structopt(name = "add")]
    Add {
        #[structopt(name = "url")]
        url: Url,
        #[structopt(short = "t", long = "tags")]
        tags: Vec<String>,
    }
}
