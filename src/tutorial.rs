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
		for _lesson in &LESSONS {

		}
	}
}