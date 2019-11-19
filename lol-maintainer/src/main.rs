#[macro_use]
extern crate failure;

pub mod endpoints;
mod fetch;
mod error;

pub use self::error::*;
pub use self::fetch::*;

use clap::{
    crate_authors, crate_description, crate_name, crate_version,
    App, AppSettings, Arg, SubCommand,
};
// use failure::Error;
use hyper::rt::{lazy, Future};
use log::{debug};

fn main() {
    pretty_env_logger::init();
    let matches = build_app().get_matches();

    let api = FetchApi::new(
        FetchApiConfig::new(None),
    );

    match matches.subcommand() {
        (CMD_BUILD, Some(_matches)) => {
            println!("Building code!");
            tokio::run(lazy(move || {
                api.constants().get_seasons()
                .map_err(|e| {
                    debug!("Something failed: {:?}", e)
                })
                .and_then(|_| Ok(()))
            }));
        },
        _ => {
            matches.usage(); // but unreachable
        }
    }
}

const CMD_BUILD: &str = "build";
const ARG_BUILD_CONSTANTS: &str = "constants";

fn build_app<'a>() -> App<'a, 'a> {
    App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .setting(AppSettings::SubcommandRequired)
        .subcommand(
            SubCommand::with_name(CMD_BUILD).about("Build code for lol-api")
            .arg(
                Arg::with_name(ARG_BUILD_CONSTANTS)
                .help("Generates game constants")
                .required(false)
                .index(1)
            )
        )
}
