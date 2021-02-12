# jacarex

`jacarex` is a small command-line tool for interactive regular expression testing (similar to many web-based tools), also (TODO!) featuring a small tutorial for regex-learning.

## Build

A Rust toolchain must be installed to build Jacarex. If you don't have one installed, get it at [rustup.rs](https://rustup.rs/)
Jacarex is currently being developed in `rustc v 1.5.0`, but should be buildable on considerably older versions as well.

```bash
git clone https://github.com/vrmiguel/jacarex   # Clone the repo
cd jacarex && cargo run --release               # And build it
```

## Usage



## Notes

`jacarex` uses the excellent [`regex`](https://github.com/rust-lang/regex) crate, and therefore inherits its advantages, such as great performance, as well as its quirks (no backtracking, no look around, no UTF-16 support).

## 
