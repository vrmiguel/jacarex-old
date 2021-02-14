use colored::*;

use crate::cliargs::PlaygroundArgValues::{self, *};
use crate::prompt::Prompt;
use crate::regexattempt::RegexAttempt;
use crate::text::Text::{self, *};
use crate::utils;

struct PlaygroundData {
    pub test_strings: Vec<Text>,
    pub editor: Prompt,
}

impl PlaygroundData {
    fn new() -> Self {
        Self {
            test_strings: vec![],
            editor: Prompt::new(),
        }
    }

    fn print_help() {
        todo!()
    }

    fn use_arg_values(&mut self, values: Option<PlaygroundArgValues>) {
        if let Some(arg_vals) = values {
            match arg_vals {
                Words(words) => {
                    print!("Loaded ");
                    words
                        .into_iter()
                        .for_each(|x| self.test_strings.push(Word(x)));
                    for text in self.test_strings.iter() {
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

    fn parse(&mut self, line: &str) {
        // TODO: add commands such as #addword and #addline
        let line = line.trim();
        if line.starts_with('#') {
            // Likely a command, such as #help or #add
            let words: Vec<&str> = line.split(' ').collect();
            match line {
                line if line.starts_with("#help") => Self::print_help(),
                line if line.starts_with("#clear") => {
                    self.test_strings.clear();
                    return;
                }
                line if line.starts_with("#addword") => {
                    words
                        .iter()
                        .skip(1)
                        .for_each(|&s| self.test_strings.push(Word(s.into())));
                    return;
                }
                line if line.starts_with("#addline") => {
                    self.test_strings.push(Line(String::from(line)));
                    return;
                }
                // "#readfile" => {     // <- I'm unsure if I should add this one
                // },
                _ => {}
            }
        }

        // utils::print_captures(line, &self.test_strings);
        match RegexAttempt::new(line, &*self.test_strings) {
            Ok(attempt) => {
                attempt.print_matches();
            }
            Err(err) => utils::show_regex_error(err),
        }
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

    /// Starts the Playground loop.
    pub fn start(arg_values: Option<PlaygroundArgValues>) {
        PlaygroundManager::intro_text();
        let mut data = PlaygroundData::new();
        data.use_arg_values(arg_values);
        loop {
            match data.editor.read_line(">> ") {
                Ok(line) => data.parse(line.as_str()),
                Err(err) => {
                    // Prints some additional info depending on which error we're getting
                    utils::check_readline_error(err);
                    data.editor.save_history();
                    return;
                }
            }
        }
    }
}
