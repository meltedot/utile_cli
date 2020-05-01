# utile_cli
Useful abstractions for Rust's pancurses. The aim is to ease pancurses development by adding a series of useful simple and documented functions and to allow for simpler terminal-based applications.

## Documentation
## Requirements
Visit the [pancurses](https://github.com/ihalila/pancurses) repository

## Usage
Visit the [documentation](https://docs.rs/utile_cli/0.3.1/utile_cli/)

Cargo.toml
```toml
[dependencies]
utile_cli = "0.3.1"
```

main.rs
```rust
extern crate utile_cli;

use utile_cli::cli::{Terminal};

fn main() {
  let term = Terminal::new();
  term.outln("Hello world!");
}
```