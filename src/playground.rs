use crate::prompt::Prompt;
use crate::utils;
use crate::cliargs::PlaygroundArgValues::{self, *};

use colored::*;

struct PlaygroundData {
    pub phrases: Vec<String>,
    pub editor: Prompt,
}

impl PlaygroundData {    
    fn new () -> Self {
        Self {
            phrases: vec![],
            editor: Prompt::new()
        }
    }

    fn print_help() {
        println!();
        println!();
    }

    fn parse (&mut self, line: &str) {
        let line = line.trim();
        if line.starts_with('#') {
            // Likely a command, such as #help or #add
            let words: Vec<&str> = line.split(' ').collect();
            match words[0] {
                "#help" => { 
                    println!("help heree")  
                },
                "#add" => {
                    // TODO: add support for "# add" as well
                    words.iter().skip(1).for_each(|&s| self.phrases.push(s.into()));
                    return;
                },
                "#read" => {
                    // TODO: read a file text into Jacarex
                },
                "#clear" => {
                    self.phrases.clear();
                    return;
                }
                _ => {}
            }
        }

        utils::print_captures(line, &self.phrases);
    }
}

pub struct PlaygroundManager {}

impl PlaygroundManager {
    /// Prints the introductory playground text
    fn intro_text() {
        println!("Welcome to the {} {}. Type in {} to get additional help. \nType in {} {} to add a new string as a test target.",
            "Jacarex".green(),
            "Playground".green().bold(),
            "#help".blue(),
            "#add".blue(),
            "<strings>".blue().bold(),
        )
    }

    fn use_arg_values(values: Option<PlaygroundArgValues>, data: &mut PlaygroundData) {
        if let Some(arg_vals) = values {
            match arg_vals {
                Words(words) => {
                    print!("Loaded ");
                    words.into_iter().for_each(|x| data.phrases.push(x));
                    for str in data.phrases.iter() {
                        print!("{} ", str.blue());
                    }
                    println!("from {}.", "`-w/--words`".blue().bold());
                }
                Filename(_filename) => {
                    // TODO: implement reading the file and loading it 
                    todo!()
                }
            }
        }
        // drop(values);
    }

    /// Starts the Playground loop.
    pub fn start(arg_values: Option<PlaygroundArgValues>) {
        PlaygroundManager::intro_text();
        let mut data = PlaygroundData::new();
        PlaygroundManager::use_arg_values(arg_values, &mut data);
        loop {
            match data.editor.read_line(">> ") {
                Ok(line) => data.parse(line.as_str()),
                Err(err) => {
                    // Prints some additional info depending on which error we're getting
                    utils::check_error(err);
                    data.editor.save_history();
                    return;
                }
            }
        }
    }
} 