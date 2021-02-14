use regex::Regex;

use colored::*;

use crate::text::Text::{self, *};

/// RegexTry represents a single 'try' of a selection of test strings against a single regex rule
pub struct RegexAttempt<'t> {
    test_strings: &'t [Text],
    captures: Vec<Option<regex::Captures<'t>>>
}

impl<'t> RegexAttempt<'t> {

    pub fn new(regex_rule: &str, test_strings: &'t [Text]) -> Result<Self, regex::Error> {

        let re = Regex::new(regex_rule)?;

        let captures : Vec<Option<regex::Captures<'t>>> =
            test_strings
            .iter()
            .map(|phrase| re.captures(phrase.as_str()))
            .collect();

        Ok(Self {
            test_strings,
            captures
        })
    }

    fn print_highlights(matches: &Vec<regex::Match>, phrase: &Text) {
        let mut ranges = matches.iter().map(|x: &regex::Match| x.range());
    
        let (is_line, phrase) = match phrase {
            Word(word) => (false, word),
            Line(line) => (true, line),
        };
    
        if is_line {
            print!("\"")
        }
    
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
    
        if is_line {
            println!("\"")      
        } else {
            println!("");
        }
    }

    pub fn print_matches(&self) {
        for (phrase, capture) in self.test_strings.iter().zip(self.captures.iter()) {
            match capture {
                Some(cap) => {
                    let matches: Vec<regex::Match> = cap.iter().filter_map(|x| x).collect();
                    Self::print_highlights(&matches, phrase);
                }
                None => {
                    // Signal lack of match by issuing a red color
                    match phrase {
                        Word(word) => println!("{}", word.as_str().red()),
                        Line(line) => println!("\"{}\"", line.as_str().red())
                    };       
                }
            }
        }
    }
}