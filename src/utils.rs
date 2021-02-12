use rustyline::error::ReadlineError;

use regex::Regex;
use colored::*;

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
                // TODO: this is slow :/ 
                // I really need to find a better way of highlighting matches
                let matches: Vec<regex::Match> = cap.iter().filter_map(|x| x).collect();
                highlight_matches(&matches, phrase);
            }
            None => {
                // Signal lack of match by issuing a red color
                println!("{}", phrase.red())
            }
        }
    }

    // let captures = RE.captures(word);
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

// TODO: pls do this better
fn highlight_matches(matches: &Vec<regex::Match>, phrase: &String) {
    // let res = String::new();
    // Naming this match_ to avoid usage of reserved keyword match
    println!("{}", phrase);
    let t = matches[0];
    let t = t.range();
    dbg!(t);
    // let t = t.
    // for match_ in matches {
    //     let phrase: Vec<char> = phrase.chars().collect();
    //     let mut match_counter = 0_usize;

    //     for ch in match_.as_str().chars() {
    //         if ch == phrase[match_counter] {
    //             // res += ;
    //         }
    //     }
    // }
}