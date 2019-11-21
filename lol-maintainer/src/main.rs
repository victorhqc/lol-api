#[macro_use]
extern crate failure;

pub mod endpoints;
mod error;
mod fetch;

pub use self::error::*;
pub use self::fetch::*;

use endpoints::ToDescriptor;
use std::env;

use clap::{
    crate_authors, crate_description, crate_name, crate_version, App, AppSettings, Arg, SubCommand,
};
// use failure::Error;
use handlebars::Handlebars;
use hyper::rt::{lazy, Future};
use log::debug;

fn main() {
    pretty_env_logger::init();
    let matches = build_app().get_matches();

    let api = FetchApi::new(FetchApiConfig::new(None));

    match matches.subcommand() {
        (CMD_BUILD, Some(matches)) => {
            println!("Building code");
            let constants = matches.value_of(ARG_BUILD_CONSTANTS);

            match constants {
                Some(_) => {
                    tokio::run(lazy(move || {
                        let mut reg = Handlebars::new();
                        reg.register_template_file(
                            "constant_file",
                            "./lol-maintainer/src/partials/constants.hbs",
                        )
                        .unwrap();

                        let seasons_request = api.constants().get_seasons();

                        seasons_request
                            .map_err(|e| debug!("Something failed: {:?}", e))
                            .and_then(move |seasons| {
                                println!("Found {} seasons", seasons.items.len());

                                for season in seasons.items.iter() {
                                    println!("{:?}", season);
                                }

                                let r = reg.render("constant_file", &seasons.descriptor()).unwrap();
                                println!("{}", r);

                                Ok(())
                            })
                    }));
                }
                None => {}
            }
        }
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
            SubCommand::with_name(CMD_BUILD)
                .about("Build code for lol-api")
                .arg(
                    Arg::with_name(ARG_BUILD_CONSTANTS)
                        .help("Generates game constants")
                        .required(false),
                ),
        )
}
