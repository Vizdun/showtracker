extern crate clap;

use clap::{
    App,
    AppSettings,
    Arg
};

mod commands;
mod common;
mod storage;
mod structs;

use commands::{
    check::*,
    list::*,
    search::*,
    track::*,
    untrack::*,
    update::*
};

fn main() {

    let matches = App::new("Show Tracker")
        .version("1.0.0")
        .setting(AppSettings::ArgRequiredElseHelp)
        .subcommand(
            App::new("check").about(
                "Checks if there are any new episodes"
            )
        )
        .subcommand(
            App::new("update")
                .about("Updates the show list")
        )
        .subcommand(
            App::new("search")
                .about("Searches the show list")
                .arg(
                    Arg::with_name("TERM")
                        .help("Search term")
                        .required(true)
                        .index(1)
                )
        )
        .subcommand(
            App::new("track")
                .about("Starts tracking a show")
                .arg(
                    Arg::with_name("SHOW")
                        .help("Show ID")
                        .required(true)
                        .index(1)
                )
        )
        .subcommand(
            App::new("untrack")
                .about("Stops tracking a show")
                .arg(
                    Arg::with_name("SHOW")
                        .help("Show ID")
                        .required(true)
                        .index(1)
                )
        )
        .subcommand(
            App::new("list").about("Lists tracked shows")
        )
        .get_matches();

    match matches.subcommand().0 {
        "update" => update_show_list(),
        "search" => {
            search_shows(
                matches
                    .subcommand_matches("search")
                    .unwrap()
                    .value_of("TERM")
                    .unwrap()
            )
        }
        "track" => {
            track_show(
                matches
                    .subcommand_matches("track")
                    .unwrap()
                    .value_of("SHOW")
                    .unwrap()
            )
        }
        "untrack" => {
            untrack_show(
                matches
                    .subcommand_matches("untrack")
                    .unwrap()
                    .value_of("SHOW")
                    .unwrap()
            )
        }
        "list" => list_tracked(),
        "check" => check_for_new_episodes(),
        _ => {}
    }
}
