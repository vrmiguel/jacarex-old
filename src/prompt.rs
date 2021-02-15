use std::collections::HashSet;

use rustyline::hint::{Hint, Hinter};
use rustyline::Context;
use rustyline::Editor;
use rustyline_derive::{Completer, Helper, Highlighter, Validator};
use rustyline::error::ReadlineError;

/// Simple wrapper over a rustyline::Editor
pub struct Prompt {
    editor: rustyline::Editor<EditorHinter>,
}

#[derive(Hash, Debug, PartialEq, Eq)]
struct CommandHint {
    display: String,
    complete_up_to: usize,
}

#[derive(Completer, Helper, Validator, Highlighter)]
struct EditorHinter {
    hints: HashSet<CommandHint>,
}

pub enum ManagerMode {
    Playground,
    Tutorial
}

impl CommandHint {
    fn new(text: &str, complete_up_to: &str) -> CommandHint {
        assert!(text.starts_with(complete_up_to));
        CommandHint {
            display: text.into(),
            complete_up_to: complete_up_to.len(),
        }
    }

    fn suffix(&self, strip_chars: usize) -> CommandHint {
        CommandHint {
            display: self.display[strip_chars..].to_owned(),
            complete_up_to: self.complete_up_to.saturating_sub(strip_chars),
        }
    }
}

fn playground_hints() -> HashSet<CommandHint> {
    let mut set = HashSet::new();
    set.insert(CommandHint::new("#help",     "#h"));
    set.insert(CommandHint::new("#clear",    "#c"));
    set.insert(CommandHint::new("#readfile", "#r"));
    set.insert(CommandHint::new("#addline",  "#addl"));
    set.insert(CommandHint::new("#addword",  "#addw"));
    set
}

fn tutorial_hints() -> HashSet<CommandHint> {
    // TODO: add more stuff
    let mut set = HashSet::new();
    set.insert(CommandHint::new("#help", "#h"));
    set
}

impl Hinter for EditorHinter {
    type Hint = CommandHint;

    fn hint(&self, line: &str, pos: usize, _ctx: &Context<'_>) -> Option<CommandHint> {
        if pos < line.len() {
            return None;
        }

        self.hints
            .iter()
            .filter_map(|hint| {
                if pos > 0 && hint.display.starts_with(&line[..pos]) {
                    Some(hint.suffix(pos))
                } else {
                    None
                }
            })
            .next()
    }
}


impl Hint for CommandHint {
    fn display(&self) -> &str {
        &self.display
    }

    fn completion(&self) -> Option<&str> {
        if self.complete_up_to > 0 {
            Some(&self.display[..self.complete_up_to])
        } else {
            None
        }
    }
}

impl Prompt {
    /// Returns a new rustyline::Editor with history loaded in (if it exists)
    pub fn new(mode: ManagerMode) -> Self {
        let mut editor = Editor::<EditorHinter>::new();
        let hinter = EditorHinter {
            hints: match mode {
                ManagerMode::Tutorial   => tutorial_hints(),
                ManagerMode::Playground => playground_hints()
            }
        };
        editor.set_helper(Some(hinter));
        if editor.load_history("history.txt").is_err() {}

        Self { editor }
    }

    pub fn read_line(&mut self, prompt: &str) -> Result<String, ReadlineError> {
        match self.editor.readline(prompt) {
            Ok(line) => {
                self.editor.add_history_entry(line.as_str());
                Ok(line)
            }
            err => err,
        }
    }

    pub fn save_history(&mut self) {
        if let Err(err) = self.editor.save_history("history.txt") {
            eprintln!("Warning: problem saving history: {:?}", err);
        }
    }
}
