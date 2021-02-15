use colored::*;

pub struct REHelp {}

pub enum REHelpKind {
    All,
    Characters,
    Quantifiers,
    Logic,

}

impl REHelp {

    pub fn new(kind: REHelpKind) {
        use REHelpKind::*;
        match kind {
            Characters => {
                Self::on_characters();
            }
            Quantifiers => {
                Self::on_quantifiers();
            }
            Logic => {
                todo!();
            }
            All => {
                Self::on_characters();
                Self::on_quantifiers();
            }
        }
    }

    fn on_quantifiers() {
        println!("Quantifiers specify how many instances of a character, group, or character class must be present in the input for a match to be found.");
        println!("{} - Match one or more times", "+".blue());
        println!("{} - Match zero or more times", "*".blue());
        println!("{} - Match zero or one time", "?".blue());
        println!("{} - Match `n` times", "{n}".blue());
        println!("{} - Match at least `n` times", "{n,}".blue());
        println!("{} - Match from `n` to `m` times", "{n,m}".blue());
        println!("{} can also make quantifiers lazy", "?".blue());
        println!("\t Ex.: on the string \"2345\",`{}` matches \"{}\", while `{}` matches \"{}45\"", 
                "\\w{2,4}".bright_white(),
                "2345".green(),
                "\\w{2,4}?".bright_white(),
                "23".green()
        );
    }

    fn on_characters() {
        println!("{} - Matches any character except newlines", ".".blue());
        println!("{} - Matches one digit from 0 to 9", "\\d".blue());
        println!("{} - Matches a character - ASCII or valid UTF-8", "\\w".blue());
        println!("{} - Matches a whitespace character", "\\s".blue());
        println!("{} - Matches a character that is not a digit", "\\D".blue());
        println!("{} - Matches a character that is not part of the alphabet", "\\W".blue());
        println!("{} - Matches a character that is not whitespace.", "\\S".blue());
        println!("{} - Escapes a special character, such as in `\\.` or `", "\\\\".blue());
    }
}