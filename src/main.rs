extern crate clap;
#[macro_use]
extern crate tantivy;

use clap::{App, AppSettings, Arg};

mod commands;
mod common;
mod storage;
mod structs;

fn main() {
    let matches = App::new("Show Tracker")
        .version("Development Version")
        .setting(AppSettings::ArgRequiredElseHelp)
        .subcommand(
            App::new("check").about(
                "Checks if there are any new episodes",
            ),
        )
        .subcommand(
            App::new("update")
                .about("Updates the show list"),
        )
        .subcommand(
            App::new("search")
                .about("Searches the show list")
                .arg(
                    Arg::with_name("TERM")
                        .help("Search term")
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::with_name("max")
                        .help("Maximum number of results")
                        .short("m")
                        .long("max")
                        .default_value("10")
                        .takes_value(true),
                ),
        )
        .subcommand(
            App::new("track")
                .about("Starts tracking a show")
                .arg(
                    Arg::with_name("SHOW")
                        .help("Show ID")
                        .required(true)
                        .index(1),
                ),
        )
        .subcommand(
            App::new("untrack")
                .about("Stops tracking a show")
                .arg(
                    Arg::with_name("SHOW")
                        .help("Show ID")
                        .required(true)
                        .index(1),
                ),
        )
        .subcommand(
            App::new("list").about("Lists tracked shows"),
        )
        .get_matches();

    match matches.subcommand().0 {
        "update" => commands::update::main(),
        "search" => commands::search::main(
            matches
                .subcommand_matches("search")
                .unwrap()
                .value_of("TERM")
                .unwrap(),
            matches
                .subcommand_matches("search")
                .unwrap()
                .value_of("max")
                .unwrap(),
        ),
        "track" => commands::track::main(
            matches
                .subcommand_matches("track")
                .unwrap()
                .value_of("SHOW")
                .unwrap(),
        ),
        "untrack" => commands::untrack::main(
            matches
                .subcommand_matches("untrack")
                .unwrap()
                .value_of("SHOW")
                .unwrap(),
        ),
        "list" => commands::list::main(),
        "check" => commands::check::main(),
        _ => {}
    }
}
