use rustyline::error::ReadlineError;

use regex::Regex;
use colored::*;

/// Checks for Regex matches
pub(crate) fn print_captures(line: &str, phrases: &Vec<String>) {
    
    let re = match Regex::new(line) {
        Ok(re) => re,
        Err(err) => {
            println!("Regex compilation error: {:?}", err);
            return;
        }
    };

    let captures : Vec<Option<regex::Captures>> =
        phrases
        .iter()
        .map(|phrase| re.captures(phrase.as_str()))
        .collect();

    for (phrase, capture) in phrases.iter().zip(captures.iter()) {
        match capture {
            Some(cap) => {
                let matches: Vec<regex::Match> = cap.iter().filter_map(|x| x).collect();
                highlight_matches(&matches, phrase);
            }
            None => {
                // Signal lack of match by issuing a red color
                println!("{}", phrase.red())
            }
        }
    }
}

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

fn highlight_matches(matches: &Vec<regex::Match>, phrase: &String) {
    let mut ranges = matches.iter().map(|x: &regex::Match| x.range());
    
    let mut i = 0_usize;
    while i <= phrase.len() {
        if let Some(range) = ranges.next() {
            if i < range.start {
                print!("{}", &phrase[0..range.start]);
            }
            print!("{}", &phrase[range.clone()].green());
            i = range.end;
        } else {
            // Check if there's still parts of the string that weren't printed
            print!("{}", &phrase[i..]);
            break;
        }
    }
    println!("");
}