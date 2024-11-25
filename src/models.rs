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
    pub name: String,
    pub connection: Connection,
    pub script: String
}

#[derive(Deserialize, Debug)]
pub struct Connection {
    pub host: String,
    pub port: u16,
    pub database: String,
    pub user: String,
    pub password: String
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub tasks: Vec<Task>
}
