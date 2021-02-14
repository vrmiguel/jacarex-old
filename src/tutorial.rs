use crate::prompt::Prompt;
use crate::text::Text::{self, Word};
use crate::utils;
#[allow(dead_code)]
struct Lesson {
    intro_text: &'static str,
    test_strings: [&'static str; 4]
}

#[allow(dead_code)]
const LESSONS: [Lesson; 1] = [
    // Lesson 1
    Lesson { 
        intro_text: "Placeholder text",
        test_strings: ["abc", "abcde", "abcdef", "ghef"]
    },
    // .. other lessons
];

struct TutorialManager {}

impl TutorialManager {
	fn start () {
        let mut editor = Prompt::new();
        let mut solved = false;
		for lesson in &LESSONS {
            let mut test_strings: Vec<Text> =
                lesson
                .test_strings
                .iter()
                .map(|&x| Word(x.to_string()))
                .collect();
                
            println!("{}", lesson.intro_text);

            while !solved {
                match editor.read_line(">> ") {
                    Ok(line) => {
                        
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