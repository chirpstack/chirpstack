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
use clap::{App, Arg};
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
mod sensitivity;
mod storage;
#[cfg(test)]
mod test;
mod uplink;

#[tokio::main]
async fn main() -> Result<()> {
    // read CLI
    let matches = App::new("chirpstack")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Orne Brocaar <info@brocaar.com>")
        .about("ChirpStack open-source LoRaWAN network-server")
        .arg(
            Arg::with_name("config-dir")
                .required(true)
                .short('c')
                .long("config-dir")
                .value_name("DIR")
                .multiple(false)
                .number_of_values(1)
                .help("Path to configuration directory")
                .takes_value(true),
        )
        .subcommand(App::new("configfile").about("Print the configuration template"))
        .subcommand(
            App::new("print-ds")
                .about("Print the device-session for debugging")
                .arg(
                    Arg::with_name("dev-eui")
                        .required(true)
                        .long("dev-eui")
                        .value_name("DEV_EUI")
                        .multiple(false)
                        .help("Device EUI")
                        .takes_value(true),
                ),
        )
        .subcommand(
            App::new("import-ttn-lorawan-devices")
                .about("Import TheThingsNetwork LoRaWAN devices repository")
                .arg(
                    Arg::with_name("dir")
                        .required(true)
                        .short('d')
                        .long("dir")
                        .value_name("DIR")
                        .multiple(false)
                        .number_of_values(1)
                        .help("Path to repository root")
                        .takes_value(true),
                ),
        )
        .get_matches();

    let config_dir = matches.get_one::<String>("config-dir").unwrap();
    config::load(Path::new(&config_dir))?;

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

    if matches.subcommand_matches("configfile").is_some() {
        cmd::configfile::run();
        process::exit(0);
    }

    if let Some(v) = matches.subcommand_matches("print-ds") {
        let dev_eui = v.get_one::<String>("dev-eui").unwrap();
        let dev_eui = EUI64::from_str(dev_eui).unwrap();

        cmd::print_ds::run(&dev_eui).await.unwrap();
        process::exit(0);
    }

    if let Some(v) = matches.subcommand_matches("import-ttn-lorawan-devices") {
        let dir = v.get_one::<String>("dir").unwrap();
        cmd::import_ttn_lorawan_devices::run(Path::new(dir))
            .await
            .unwrap();
        process::exit(0);
    }

    cmd::root::run().await?;

    Ok(())
}
