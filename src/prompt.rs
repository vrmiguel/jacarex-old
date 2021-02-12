use rustyline::error::ReadlineError;
use rustyline::Editor;

/// Simple wrapper over a rustyline::Editor
pub struct Prompt {
    editor: rustyline::Editor<()>
}

impl Prompt {
    /// Returns a new rustyline::Editor with history loaded in (if it exists)
    pub fn new() -> Self {
        let mut editor = Editor::<()>::new();
        if editor.load_history("history.txt").is_err() {}
        
        Self {
            editor
        }
    }

    pub fn read_line(&mut self, prompt: &str) -> Result<String, ReadlineError> {
        match self.editor.readline(prompt) {
            Ok(line) => {
                self.editor.add_history_entry(line.as_str());
                Ok(line)
            },
            err => err
        }
    }

    pub fn save_history(&mut self) {
        if let Err(err) = self.editor.save_history("history.txt") {
            eprintln!("Warning: problem saving history: {:?}", err);
        }
    }

}