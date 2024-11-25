use std::fs;
use clap::Parser;
use log::{error, info};
use tokio_postgres::NoTls;
use crate::models::{Cli, Config};

pub mod models;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let content = fs::read_to_string(&args.file)?;
    let config: Config = serde_yaml::from_str(&content)?;

    let task = config
        .tasks
        .into_iter()
        .find(|t| t.name == args.command)
        .ok_or_else(|| format!("Command {} not found in the file {}", args.command, args.file))?;

    info!("Running task [{}]", task.name);

    let conn_str = format!(
        "host={} port={} dbname={} user={} password={}",
        task.connection.host,
        task.connection.port,
        task.connection.database,
        task.connection.user,
        task.connection.password
    );

    let (client, connection) = tokio_postgres::connect(&conn_str, NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            error!("connection error: {}", e);
        }
    });

    if let Err(e) = client.batch_execute(&task.script).await {
        error!("batch execute task {} returned error: {}", task.name, e);
    } else {
        info!("Task {} execution success!", task.name);
    }

    Ok(())
}
