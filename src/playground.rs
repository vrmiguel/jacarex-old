use crate::prompt::{self, Prompt};
use crate::utils;

use regex::Regex;

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

    fn parse (&mut self, line: &str) {
        if line.trim().starts_with('#') {
            // Likely a command, such as #help or #add
            let words: Vec<&str> = line.trim().split(' ').collect();
            match words[0] {
                "#help" => { 
                    // Print help
                },
                "#add" => {
                    words.iter().skip(1).for_each(|&s| self.phrases.push(s.into()));
                    return;
                },
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
            "<string>".blue().bold(),
        )
    }

    /// Starts the Playground loop.
    pub fn start() {
        PlaygroundManager::intro_text();
        let mut data = PlaygroundData::new();
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