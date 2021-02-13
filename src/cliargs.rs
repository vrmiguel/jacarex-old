use clap::{self, Arg};

use std::env;

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
    Tutorial(Option<u8>)
}

pub fn get_matches() -> clap::ArgMatches<'static> {
    clap::App::new("jacarex")
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
                            // I'm only making this conflict to get better code ergonomics 
                            .conflicts_with("filename")
                            .help("Loads the words passed as arguments as test strings for the tester"),
                    ).arg(
                        Arg::with_name("filename")
                            .required(false)
                            .multiple(false)
                            // TODO: add a file size limit?
                            .help("Loads the words passed as arguments as test strings for the tester"),
                    )
                    .after_help("Note: tester is an alias to playground, so `jacarex tester` works as well.")
        )
        .subcommand(
            clap::SubCommand::with_name("tutorial")
                .about("Starts a set of guided interactive lessons on Regex matching.")
                .settings(&[
                    clap::AppSettings::ColoredHelp, 
                ])
                .arg(
                    // add lesson limits
                    Arg::with_name("lesson")
                        .required(false)
                        .multiple(false)
                        .help("Sets the lesson to be loaded")
                        // .possible_values()
                        .takes_value(true)
                )
        )
        .get_matches()
}

impl From<clap::ArgMatches<'static>> for CLIArgValues {
    fn from(matches: clap::ArgMatches<'static>) -> Self {
            match (matches.is_present("playground"), matches.is_present("tutorial")) {
            (true, false) => {
                // Check for playground values
                if matches.is_present("words") {
                    let values: Vec<String> = 
                        matches
                        .values_of("words")
                        .unwrap()   // Safe to unwrap because we know these args were passed
                        .map(|str| String::from(str))
                        .collect();

                    Self::Playground(Some(PlaygroundArgValues::Words(values)))
                    // Self::Playground(None)
                } else if matches.is_present("filename") {
                    // good god
                    Self::Playground(Some(PlaygroundArgValues::Filename(
                        String::from(matches.value_of("filename").unwrap())
                    )))
                } else {
                    Self::Playground(None)
                }

            }
            (false, true) => {
                if matches.is_present("lesson") {
                    let lesson = matches.value_of("lesson").unwrap().parse::<u8>().unwrap();
                    Self::Tutorial(Some(lesson))
                } else {
                    Self::Tutorial(None)
                }
            }
            (false, false) => {
                todo!()
            }
            (_, _) => {
                unreachable!();
            }
        }        

    }
}