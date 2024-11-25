use clap::Parser;
use serde::Deserialize;

#[derive(Parser, Deserialize)]
pub struct Cli {
    #[clap(short, long, default_value = "tasks.yaml")]
    pub file: String,
    #[clap(short, long)]
    pub command: String
}

#[derive(Deserialize, Debug)]
pub struct Task {
    pub(crate) name: String,
    pub(crate) connection: Connection,
    pub(crate) script: String
}

#[derive(Deserialize, Debug)]
pub struct Connection {
    pub(crate) host: String,
    pub(crate) port: u16,
    pub(crate) database: String,
    pub(crate) user: String,
    pub(crate) password: String
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub(crate) tasks: Vec<Task>
}
