use rustyline::error::ReadlineError;

use regex::Regex;
use colored::*;

pub(crate) fn check_error(err: ReadlineError) {
    match err {
        ReadlineError::Interrupted => {
            println!("SIGINT received. Exiting.");
        },
        ReadlineError::Eof => {
            println!("EOF received. Exiting.");
        },
        err => {
            eprintln!("Error: {:?}", err);
        }
    }
}

/// Checks for Regex matches
pub(crate) fn print_captures(line: &str, phrases: &Vec<String>) {
    
    let re = match Regex::new(line) {
        Ok(re) => {
            re
        },
        Err(err) => {
            println!("Regex compilation error: {:?}", err);
            return;
        }
    };

    // let t = re.captures(phrase.as_str();

    let captures : Vec<Option<regex::Captures>> =
        phrases
        .iter()
        .map(|phrase| re.captures(phrase.as_str()))
        .collect();

    for (phrase, capture) in phrases.iter().zip(captures.iter()) {
        match capture {
            Some(cap) => {
                let t = cap.iter()
            }
            None => {
                // Signal lack of match by issuing a red color
                println!("{}", phrase.red())
            }
        }
    }

    // let captures = RE.captures(word);
}