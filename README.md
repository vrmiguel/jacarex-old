# jacarex

`jacarex` is a small command-line tool for interactive regular expression testing (similar to many web-based tools), also (TODO!) featuring a small tutorial for regex-learning.

## Build

A Rust toolchain must be installed to build Jacarex. If you don't have one installed, get it at [rustup.rs](https://rustup.rs/)
Jacarex is currently being developed in `rustc v 1.5.0`, but should be buildable on considerably older versions as well.

```bash
git clone https://github.com/vrmiguel/jacarex   # clone the repo
cd jacarex && cargo run --release               # .. and build it
```

## Usage

`jacarex` is based on subcommands.

```
Simple regex tester and tutorial

USAGE:
    jacarex [SUBCOMMAND]

FLAGS:
    -h, --help       Displays this message and exits
    -V, --version    Prints version information

SUBCOMMANDS:
    help          Prints this message or the help of the given subcommand(s)
    playground    Enters the Regex testing REPL.
    tutorial      Starts a set of guided interactive lessons on Regex matching.
```

### `jacarex playground`

The `playground` subcommand opens up a REPL where one can add in test strings and interactively test regex rules against them.

#### Example

`<add example image here later>`

#### Usage help

```
jacarex-playground 
Enters the Regex testing REPL.

USAGE:
    jacarex playground [ARGS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <words>...    Loads the words passed as arguments as test strings for the tester
    <filename>    Loads the words passed as arguments as test strings for the tester

Note: tester is an alias to playground, so `jacarex tester` works as well.
``` 

### `jacarex tutorial`

The `tutorial` subcommand starts a series of interactive guided lessons on regex matching.

#### Example

`<add example image here later>`

#### Usage help

```
jacarex-tutorial 
Starts a set of guided interactive lessons on Regex matching.

USAGE:
    jacarex tutorial [lesson]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <lesson>    Sets the lesson to be loaded
```

## Notes

1. `jacarex` uses the excellent [`regex`](https://github.com/rust-lang/regex) crate, and therefore inherits its advantages, such as great performance, as well as its quirks (no backtracking, no look around, no UTF-16 support).

2. `jacarex` relies on terminal colors to highlight matches (or lack thereof).  A `no-colors` feature is planned but not currently implemented.