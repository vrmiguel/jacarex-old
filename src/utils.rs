use rustyline::error::ReadlineError;
use regex::{self, Error::*};

pub (crate) fn show_regex_error(err: regex::Error) {
    // The regex crate has nicely formatted errors, so
    // it's better to deestructure them and show their 
    // error messages rather than just using {:?}
    match err {
        Syntax(err)        => eprintln!("{}", err),
        CompiledTooBig(size_limit) => eprintln!("The compiled program exceeded the set size limit ({}).", size_limit),
        other => { 
            // The only other error variant, __Nonexhaustive *should* be unreachable here, but
            // rustc wants me to treat it anyway
            eprintln!("{:?}", other)
        }
    }
}

pub(crate) fn check_readline_error(err: ReadlineError) {
    match err {
        ReadlineError::Interrupted => {
            println!("SIGINT received. Exiting.");
        }
        ReadlineError::Eof => {
            println!("EOF received. Exiting.");
        }
        err => {
            eprintln!("Error: {:?}", err);
        }
    }
}
