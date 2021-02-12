mod prompt;
mod playground;
mod utils;
mod school;
mod cliargs;

use std::env;

use cliargs::CLIArgs;

fn main() {
    let cliargs = CLIArgs::from(env::args());
    playground::PlaygroundManager::start();
}