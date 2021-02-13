mod prompt;
mod playground;
mod utils;
mod school;
mod cliargs;

use cliargs::*;

fn main() {
    let matches = get_matches();
    let cliargs = CLIArgValues::from(matches);
    dbg!(&cliargs);
    playground::PlaygroundManager::start();
}