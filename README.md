# bf-compiler
A quick bf compiler made using Rust

## Requirements

* `gcc` or `clang`

## Building

### Using `cargo`
Requires the `cargo` buildsystem. Cargo can be installed as described in the [Rust book](https://doc.rust-lang.org/stable/book/ch01-01-installation.html)

* Clone the repository using `git clone https://github.com/Proxxa/bf-compiler.git`
* `cd bf-compiler`
* `cargo build --release`
* The executable is stored at `./target/release/bf-compiler`

### Using `rustc`
Requires the `rustc` compiler. The compiler can be installed the same way as Cargo.

* Clone the repository using `git clone https://github.com/Proxxa/bf-compiler.git`
* `cd bf-compiler`
* `rustc ./src/main.rs -o bf-compiler`
* The executable is stored at `./bf-compiler`
