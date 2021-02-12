use clap::{self, Arg};

use std::env;

pub struct CLIArgs {
    enter_playground: bool,
    enter_tutorial:   bool,
}

impl From<env::Args> for CLIArgs {
    fn from(args: env::Args) -> Self {
        let matches = clap::App::new("jacarex")
        .version("0.1.0")
        .about("Regex tester and tutorial")
        .help_message("Displays this message and exits")
        .after_help("Please file in bugs at https://github.com/vrmiguel/jacarex.")
        .settings(&[clap::AppSettings::ColoredHelp])
        .arg(
            Arg::with_name("playground")
                .long("--playground")
                .short("-p")
                .help("Enters the Regex testing REPL. Set by default if no other options are passed in.")
        ).arg(
            Arg::with_name("tutorial")
                .conflicts_with("playground")
                .long("tutorial")
                .short("-t")
                .help("Starts a set of guided interactive lessons on Regex matching.")
        )
        .get_matches_from(args);

        Self {
            enter_playground: matches.is_present("playground"),
            enter_tutorial: matches.is_present("tutorial")
        }
    }
}