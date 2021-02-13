mod prompt;
mod playground;
mod utils;
mod tutorial;
mod cliargs;

use cliargs::{get_matches, CLIArgValues::{self, *}};

fn main() {
    let matches = get_matches();
    let cliargs = CLIArgValues::from(matches);
    match cliargs {
        Playground(arg_values) => playground::PlaygroundManager::start(arg_values),
        Tutorial(_)                                    => {}
    }
    // playground::PlaygroundManager::start(cliargs);
}