use rustyline::error::ReadlineError;
// use regex::Regex;
// use colored::*;

// use crate::text::Text::{self, *};

/// Checks for Regex matches
// pub(crate) fn print_captures(line: &str, phrases: &Vec<Text>) {
    
//     let re = match Regex::new(line) {
//         Ok(re) => re,
//         Err(err) => {
//             println!("Regex compilation error: {:?}", err);
//             return;
//         }
//     };

//     let captures : Vec<Option<regex::Captures>> =
//         phrases
//         .iter()
//         .map(|phrase| re.captures(phrase.as_str()))
//         .collect();

//     for (phrase, capture) in phrases.iter().zip(captures.iter()) {
//         match capture {
//             Some(cap) => {
//                 let matches: Vec<regex::Match> = cap.iter().filter_map(|x| x).collect();
//                 highlight_matches(&matches, phrase);
//             }
//             None => {
//                 // Signal lack of match by issuing a red color
//                 match phrase {
//                     Word(word) => println!("{}", word.as_str().red()),
//                     Line(line) => println!("\"{}\"", line.as_str().red())
//                 };
                
//             }
//         }
//     }
// }

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
