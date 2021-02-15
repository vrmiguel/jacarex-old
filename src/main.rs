mod cliargs;
mod playground;
mod prompt;
mod regexattempt;
mod text;
mod tutorial;
mod utils;

use cliargs::{
    get_matches,
    CLIArgValues::{self, *},
};

fn main() {
    let matches = get_matches();
    let cliargs = CLIArgValues::from(matches);
    match cliargs {
        Playground(arg_values) => playground::PlaygroundManager::start(arg_values),
        Tutorial(arg_value)    => tutorial::TutorialManager::start(arg_value),
    }
}
