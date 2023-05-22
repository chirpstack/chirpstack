// Required by rust::table macro.
#![recursion_limit = "256"]
// TODO: remove this + all dead code.
#![allow(dead_code)]

#[macro_use]
extern crate lazy_static;
extern crate diesel_migrations;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate anyhow;

use std::path::Path;
use std::process;
use std::str::FromStr;

use anyhow::Result;
use clap::{Parser, Subcommand};
use tracing::Level;
use tracing_subscriber::{filter, prelude::*};

use lrwn::EUI64;

mod adr;
mod api;
mod backend;
mod certificate;
mod cmd;
mod codec;
mod config;
mod devaddr;
mod downlink;
mod eventlog;
mod framelog;
mod gateway;
mod gpstime;
mod integration;
mod maccommand;
mod metalog;
mod monitoring;
mod region;
mod requestlog;
mod sensitivity;
mod storage;
#[cfg(test)]
mod test;
mod uplink;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Path to configuration directory
    #[arg(short, long, value_name = "DIR")]
    config: String,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Print the configuration template
    Configfile {},

    /// Print the device-session for debugging
    PrintDs {
        /// Device EUI
        #[arg(long, value_name = "DEV_EUI")]
        dev_eui: String,
    },

    /// Import legacy lorawan-devices repository.
    ImportLegacyLorawanDevicesRepository {
        /// Path to repository root.
        #[arg(short, long, value_name = "DIR")]
        dir: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    config::load(Path::new(&cli.config))?;

    let conf = config::get();
    let filter = filter::Targets::new().with_targets(vec![
        ("chirpstack", Level::from_str(&conf.logging.level).unwrap()),
        ("backend", Level::from_str(&conf.logging.level).unwrap()),
        ("lrwn", Level::from_str(&conf.logging.level).unwrap()),
    ]);

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(filter)
        .init();

    if let Some(Commands::Configfile {}) = &cli.command {
        cmd::configfile::run();
        process::exit(0);
    }

    if let Some(Commands::PrintDs { dev_eui }) = &cli.command {
        let dev_eui = EUI64::from_str(dev_eui).unwrap();
        cmd::print_ds::run(&dev_eui).await.unwrap();
        process::exit(0);
    }

    if let Some(Commands::ImportLegacyLorawanDevicesRepository { dir }) = &cli.command {
        cmd::import_legacy_lorawan_devices_repository::run(Path::new(&dir))
            .await
            .unwrap();
        process::exit(0);
    }

    cmd::root::run().await?;

    Ok(())
}
