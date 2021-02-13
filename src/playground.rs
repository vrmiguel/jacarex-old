use crate::prompt::Prompt;
use crate::utils;
use crate::cliargs::PlaygroundArgValues::{self, *};
use crate::text::Text::{self, *};

use colored::*;

struct PlaygroundData {
    pub test_strings: Vec<Text>,
    pub editor: Prompt,
}

impl PlaygroundData {    
    fn new () -> Self {
        Self {
            test_strings: vec![],
            editor: Prompt::new()
        }
    }

    fn print_help() {
        todo!()
    }

    fn parse (&mut self, line: &str) {
        // TODO: add commands such as #addword and #addline
        let line = line.trim();
        if line.starts_with('#') {
            // Likely a command, such as #help or #add
            let words: Vec<&str> = line.split(' ').collect();
            match words[0] {
                "#help" => { 
                    println!("help heree")  
                },
                "#addword" => {
                    // TODO: add support for "# add" as well
                    words.iter().skip(1).for_each(|&s| self.test_strings.push(Word(s.into())));
                    return;
                },
                "#addline" => {
                    // TODO: don't break words in here, put the whole line in
                    words.iter().skip(1).for_each(|&s| self.test_strings.push(Line(s.into())));
                    return;
                },
                "#read" => {
                    // TODO: read a file text into Jacarex
                },
                "#clear" => {
                    self.test_strings.clear();
                    return;
                }
                _ => {}
            }
        }

        utils::print_captures(line, &self.test_strings);
    }
}

pub struct PlaygroundManager {}

impl PlaygroundManager {
    /// Prints the introductory playground text
    fn intro_text() {
        println!("Welcome to the {} {}. Type in {} to get additional help. \nType in {} {} to add new words as test targets.",
            "Jacarex".green(),
            "Playground".green().bold(),
            "#help".blue(),
            "#addword".blue(),
            "<strings>".blue().bold(),
        )
    }

    fn use_arg_values(values: Option<PlaygroundArgValues>, data: &mut PlaygroundData) {
        if let Some(arg_vals) = values {
            match arg_vals {
                Words(words) => {
                    print!("Loaded ");
                    words.into_iter().for_each(|x| data.test_strings.push(Word(x)));
                    for text in data.test_strings.iter() {
                        print!("{} ", text.as_str().blue());
                    }
                    println!("from {}.", "`-w/--words`".blue().bold());
                }
                Filename(_filename) => {
                    // TODO: implement reading the file and loading it 
                    todo!()
                }
            }
        }
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