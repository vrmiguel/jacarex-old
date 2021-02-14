use clap::{self, Arg};

use std::unreachable;

#[derive(Debug)]
pub enum PlaygroundArgValues {
    Words(Vec<String>),
    Filename(String),
}
#[derive(Debug)]
pub enum CLIArgValues {
    // May hold a Vec<String> (used when `jacarex playground --words` is used)
    //       or a String holding a filename
    Playground(Option<PlaygroundArgValues>),
    // Used when `jacarex tutorial --lesson` is used.
    // The u8 value holds the number of the lesson to be loaded.
    Tutorial(Option<u8>),
}

pub fn get_matches() -> clap::ArgMatches<'static> {
    clap::App::new("jacarex")
        .version("0.1.0")
        .about("Simple regex tester and tutorial")
        .help_message("Displays this message and exits")
        .settings(&[
            clap::AppSettings::ColoredHelp,
            clap::AppSettings::ArgRequiredElseHelp,
            clap::AppSettings::InferSubcommands,
        ])
        .subcommand(
            clap::SubCommand::with_name("playground")
                // .help("Enters the Regex testing REPL.")
                .about("Enters the Regex testing REPL.")
                .settings(&[clap::AppSettings::ColoredHelp])
                .alias("tester")
                .arg(
                    Arg::with_name("words")
                        .required(false)
                        .long("words")
                        .short("w")
                        .multiple(true)
                        .takes_value(true)
                        // I'm only making this conflict to get better code ergonomics
                        .help("Loads the words passed as arguments as test strings for the tester"),
                )
                .arg(
                    Arg::with_name("filename")
                        .required(false)
                        .long("filename")
                        .short("f")
                        .multiple(false)
                        .takes_value(true)
                        .conflicts_with("words")
                        // TODO: add a file size limit?
                        .help("Loads the words passed as arguments as test strings for the tester"),
                )
                .after_help(
                    "Note: tester is an alias to playground, so `jacarex tester` works as well.",
                ),
        )
        .subcommand(
            clap::SubCommand::with_name("tutorial")
                .about("Starts a set of guided interactive lessons on Regex matching.")
                .settings(&[clap::AppSettings::ColoredHelp])
                .arg(
                    // add lesson limits
                    Arg::with_name("lesson")
                        .required(false)
                        .multiple(false)
                        .long("lesson")
                        .short("l")
                        .help("Sets the lesson to be loaded")
                        // .possible_values()
                        .takes_value(true),
                ),
        )
        .get_matches()
}

impl From<clap::ArgMatches<'static>> for CLIArgValues {
    fn from(matches: clap::ArgMatches<'static>) -> Self {
        use CLIArgValues::*;
        use PlaygroundArgValues::*;
        match matches.subcommand() {
            ("playground", Some(playground_matches)) => {
                if let Some(words) = playground_matches.values_of("words") {
                    let values: Vec<String> = words.map(|str| String::from(str)).collect();

                    Playground(Some(Words(values)))
                } else if let Some(filename) = playground_matches.value_of("filename") {
                    Playground(Some(Filename(String::from(filename))))
                } else {
                    Playground(None)
                }
            }

            ("tutorial", Some(tutorial_matches)) => {
                if let Some(lesson) = tutorial_matches.value_of("lesson") {
                    let lesson = lesson.parse::<u8>().unwrap();
                    Tutorial(Some(lesson))
                } else {
                    Tutorial(None)
                }
            }
            _other => {
                unreachable!();
            }
        }
    }
}
