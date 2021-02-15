# jacarex

`jacarex` is a small command-line tool for interactive regular expression testing (similar to many web-based tools), also featuring a small tutorial for regex learning.

## Build

A Rust toolchain must be installed to build Jacarex. If you don't have one installed, get it at [rustup.rs](https://rustup.rs/)

Jacarex is currently being developed with `rustc v1.5.0`, but should be buildable on considerably older versions as well.

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

![regex-playground-example](https://user-images.githubusercontent.com/36349314/107896510-363e9700-6f15-11eb-92fb-b4aad8c1ee76.png)

#### Usage help

```
jacarex-playground 
Enters the Regex testing REPL.

USAGE:
    jacarex playground [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -f, --filename <filename>    Loads the file passed as argument as a test string
    -w, --words <words>...       Loads the words passed as arguments as test strings

Note: tester is an alias to playground, so `jacarex tester` works as well.
``` 

### `jacarex tutorial`

The `tutorial` subcommand starts a series of interactive guided lessons on regex matching.

#### Example

![regex-tutorial-example](https://user-images.githubusercontent.com/36349314/107896513-376fc400-6f15-11eb-95b2-ae24b29dd38b.png)

#### Usage help

```
jacarex-tutorial 
Starts a set of guided interactive lessons on Regex matching.

USAGE:
    jacarex tutorial [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

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
