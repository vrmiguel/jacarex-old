use rustyline::error::ReadlineError;

mod prompt;
mod playground;
mod utils;

use prompt::Prompt;


fn main() {


    
    // let mut rl = Prompt::new();

    // loop {
    //     match rl.read_line(">> ") {
    //         Ok(line) => {
    //             // rl.add_history_entry(line.as_str());
    //             println!("Line: {}", line);
    //         },
    //         Err(ReadlineError::Interrupted) => {
    //             println!("SIGINT received. Exiting.");
    //             break
    //         },
    //         Err(ReadlineError::Eof) => {
    //             println!("CTRL-D");
    //             break
    //         },
    //         Err(err) => {
    //             println!("Error: {:?}", err);
    //             break
    //         }
    //     }
    // }
    // rl.save_history();

    playground::PlaygroundManager::start();

}