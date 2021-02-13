use clap::{self, Arg};

use std::env;

#[derive(Debug)]
pub struct CLIArgs {
    enter_playground: bool,
    enter_tutorial:   bool,
}

impl From<env::Args> for CLIArgs {
    fn from(args: env::Args) -> Self {
        let matches = clap::App::new("jacarex")
        .version("0.1.0")
        .about("Simple regex tester and tutorial")
        .help_message("Displays this message and exits")
        .settings(&[
            clap::AppSettings::ColoredHelp, 
            clap::AppSettings::ArgRequiredElseHelp,
            clap::AppSettings::InferSubcommands
        ])
        .subcommand(
        clap::SubCommand::with_name("playground")
                    // .help("Enters the Regex testing REPL.")
                    .about("Enters the Regex testing REPL.")
                    .settings(&[
                        clap::AppSettings::ColoredHelp, 
                    ])
                    .alias("tester")
                    .arg(
                        Arg::with_name("words")
                            .required(false)
                            .multiple(true)
                            .help("Loads the words passed as arguments as test strings for the tester"),
                    )
        )
        .subcommand(
            clap::SubCommand::with_name("tutorial")
                .about("Starts a set of guided interactive lessons on Regex matching.")
        )
        .get_matches_from(args);

        Self {
            enter_playground: matches.is_present("playground"),
            enter_tutorial: matches.is_present("tutorial")
        }
    }
}