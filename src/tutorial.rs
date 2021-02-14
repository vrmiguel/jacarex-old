use colored::Colorize;

use crate::text::Text::{self, Word};
use crate::utils;
use crate::{prompt::Prompt, regexattempt::RegexAttempt};
#[allow(dead_code)]
struct Lesson {
    /// The text to be shown to the user at the start of the lesson
    intro_text: &'static str,
    /// The strings the user will have to fully match in order to progress
    /// into the next lesson
    test_strings: [&'static str; 3],
    test_kinds: [TestKind; 3],
    /// Some small text congratulating the user for concluding the lesson
    congratulations: &'static str,
}

#[allow(dead_code)]
const LESSONS: [Lesson; 2] = [
    // Lesson 1
    Lesson {
        intro_text: "Part 1: Learning about anchors\nPlaceholder explanation",
        test_strings: ["The Daily News", "The Amazing Spider-Man", "A Boy"],
        test_kinds: [TestKind::Match, TestKind::Match, TestKind::Skip],
        congratulations: "You did it!",
    },
    // Lesson 2
    Lesson {
        intro_text: "Lesson 2 placeholder text",
        test_strings: ["abc123xyz", "define \"123\"", "var g = 123;"],
        test_kinds: [TestKind::Match; 3],
        congratulations: "Congrats!",
    },
];
pub struct TutorialManager {}

#[derive(Debug, Clone, Copy)]
pub enum TestKind {
    Match,
    Skip,
}

impl TutorialManager {

    fn print_lesson_intro(lesson: &Lesson) {
        use TestKind::*;
        println!("{}", lesson.intro_text);
            print!("You'll have to pass these tests: ");
            for (test_string, test_kind) in lesson.test_strings.iter().zip(lesson.test_kinds.iter()) {
                match test_kind {
                    Match => {
                        print!("{} ", test_string.green());
                    },
                    Skip => {
                        print!("{} ", test_string.red());
                    }
                }
            }
            println!("\nStrings in {} have to be fully matched. Strings in {} have to be fully skipped.", 
                "green".green(), "red".red());
    }

    pub fn start(_arg_val: Option<u8>) {
        // TODO: use arg_val
        let mut editor = Prompt::new();
        for lesson in &LESSONS {
            let test_strings: Vec<Text> = lesson
                .test_strings
                .iter()
                .map(|&x| Word(x.to_string()))
                .collect();

            Self::print_lesson_intro(lesson);

            // Starts the read-eval-print loop. It only stops if the user closes the process
            // or if the user concludes the lesson, which would lead to the next lesson
            'repl: loop {
                match editor.read_line(">> ") {
                    Ok(line) => match RegexAttempt::new(&line, &*test_strings) {
                        Ok(attempt) => {
                            attempt.print_matches();
                            if attempt.passed_all_tests(&lesson.test_kinds) {
                                println!("{}", lesson.congratulations.green().bold());
                                break 'repl;
                            }
                        }
                        Err(err) => eprintln!("Problem compiling regex: {:?}", err),
                    },
                    Err(err) => {
                        // Prints some additional info depending on which error we're getting
                        utils::check_error(err);
                        editor.save_history();
                        return;
                    }
                }
            }
        }
    }
}
