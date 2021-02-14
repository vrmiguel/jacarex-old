pub enum Text {
    Word(String),
    Line(String),
}

impl Text {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Word(word) => word,
            Self::Line(line) => line,
        }
    }
}
