use std::fs;

use colored::*;

use crate::cliargs::PlaygroundArgValues::{self, *};
use crate::prompt::{Prompt, ManagerMode};
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
            editor: Prompt::new(ManagerMode::Playground),
        }
    }

    fn print_help() {
        println!("{} receives a word (or a list of word) and adds it to the set of test targets. Note that this mode trims its words.", "#addword".blue());
        println!("{} adds the entire line after the command as a test target. Trailing whitespace will be kept.", "#addline".blue());
        println!("{} reads the indicated file and saves it as a single test target.", "#readfile".blue());
        println!("{} clears all loaded test strings. If you just want to clean your terminal, Ctrl+L works.", "#clear".blue());
    }

    fn load_from_file(&mut self, filename: &str) {
        match fs::read_to_string(filename) {
            Ok(contents) => {
                self.test_strings.push(Text::Line(contents));
            },
            Err(err) => {
                eprintln!("Problem reading file: {:?}", err);
            }
        }
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
                Filename(filename) => {
                    self.load_from_file(&filename);
                }
            }
        }
    }

    fn parse(&mut self, line: &str) {
        // TODO: add commands such as #addword and #addline
        let line = line.trim_start();
        if line.starts_with('#') {
            // Likely a command, such as #help or #add
            let words: Vec<&str> = line.split(' ').collect();
            match line {
                line if line.starts_with("#help") =>  {
                    Self::print_help();
                    return;
                }
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
                    if line.len() > 9 {
                        self.test_strings.push(Line(String::from(&line[9..])))
                    }
                    return;
                }
                line if line.starts_with("#readfile") => {
                    if line.len() > 10 {
                        self.load_from_file(&line[10..]);
                    }
                    return;
                },
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
