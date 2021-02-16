# jacarex

`jacarex` is a fast command-line tool for testing regular expressions _interactively_. \
It also features a small tutorial for regex learning.

## Installation

You need the Rust compiler to build `jacarex`. Get it at [rustup.rs](https://rustup.rs/)

Jacarex is being developed with the current `rustc v1.50` version, but should be buildable on other recent versions as well.

```bash
git clone https://github.com/vrmiguel/jacarex   # clone the repo
cargo install --path jacarex                    # Install it
```

## Usage

`jacarex` usage is based on subcommands (similar to `git`).

The subcommands available are `playground` and `tutorial`.

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

Opens up a REPL where you can test a pattern against multiple text samples.

#### Example

![regex-playground-example](https://user-images.githubusercontent.com/36349314/107896510-363e9700-6f15-11eb-92fb-b4aad8c1ee76.png)

#### Usage help

```
jacarex-playground 
Enters the Regex testing REPL.

USAGE:
    jacarex playground [OPTIONS]

OPTIONS:
    -f, --filename <filename>    Loads the file passed as argument as a test string
    -w, --words <words>...       Loads the words passed as arguments as test strings

Note: tester is an alias to playground, so `jacarex tester` works as well.
``` 

### `jacarex tutorial`

The `tutorial` subcommand starts a series of interactive guided lessons on regex matching.

![regex-tutorial-example](https://user-images.githubusercontent.com/36349314/107896513-376fc400-6f15-11eb-95b2-ae24b29dd38b.png)

#### Usage help

```
jacarex-tutorial 
Starts a set of guided interactive lessons on Regex matching.

USAGE:
    jacarex tutorial [OPTIONS]

OPTIONS:
    -l, --lesson <lesson>    Sets the lesson to be loaded
```

## Notes

1. `jacarex` uses the excellent [`regex`](https://github.com/rust-lang/regex) crate, and therefore inherits its advantages, such as great performance, as well as its quirks (no backtracking, no look around, no UTF-16 support).

2. `jacarex` relies on terminal colors to highlight matches (or lack thereof).  A `no-colors` feature is planned but not currently implemented.

## Contributions

Any contributions are welcome! 

Specially in making new lesson for the tutorial

Please file bugs on the [Issues page](https://github.com/vrmiguel/jacarex/issues/new)
