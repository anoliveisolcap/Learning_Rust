  &#xa0;

</div>

<h1 align="center"> <img src="https://doc.rust-lang.org/stable/book/img/ferris/does_not_compile.svg" alt="rust" width="100" height="100"> Learning Rust <img src="https://doc.rust-lang.org/stable/book/img/ferris/panics.svg" alt="rust" width="100" height="100"> </h1>

<p align="center">
  <img alt="Github top language" src="https://img.shields.io/github/languages/top/anoliveisolcap/learning_rust?color=ff6600">

  <img alt="Github language count" src="https://img.shields.io/github/languages/count/anoliveisolcap/learning_rust?color=ff6600">

  <img alt="Repository size" src="https://img.shields.io/github/repo-size/anoliveisolcap/learning_rust?color=ff6600">

</p>

<p align="center">
  <a href="#about">About</a> &#xa0; | &#xa0; 
  <a href="#technologies">Technologies</a> &#xa0; | &#xa0;
  <a href="#requirements">Requirements</a> &#xa0; | &#xa0;
  <a href="#starting">Starting</a> &#xa0; | &#xa0;
  <a href="https://github.com/anoliveisolcap" target="_blank">Author</a>
</p>

<br>

## About ##

Project to learn rust using this magic [book](https://doc.rust-lang.org/stable/book/print.html)

## Technologies ##

The following tools were used in this project:

- [Rust](https://www.rust-lang.org/) 


## Requirements ##

Before starting, you need to have [Rust](https://www.rust-lang.org/) installed.

## Starting ##

```bash
# Clone this project
$ git clone https://github.com/anoliveisolcap/learning_rust

# Access
$ cd learning_rust

# Install rust
$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```
## Without Cargo
```bash
# hello_world directory
cd hello_world

# To compile
make build
# or
rustc hello_world.rs

# To compile and execute
make run
# or
./hello_world.rs

# Autoformatted rust (NO!)
rustfmt hello_world.rs

# ! is a macro
```
## With Cargo
```bash
# Cargo
cargo new hello_cargo (--vcs=git)

# hello_cargo directory
cd hello_cargo

# To compile
cargo build

# To compile and execute
cargo run

# To check if your code compiles without an executable
cargo check

# To compile your project when it is finally ready for release
cargo build --release

```
Rust [keywords](https://doc.rust-lang.org/stable/book/appendix-01-keywords.html)
Rust [operators](https://doc.rust-lang.org/stable/book/appendix-02-operators.html)

Variables types:
- String

- Integer types:

| Length | Signed | Unsigned | Until |
| ------ | ------ | -------- | ----- |
| 8-bit | i8 | u8 | 128 |
| 16-bit | i16 | u16 | 32,768 |
| 32-bit | i32 | u32 | 2,147,483,648 |
| 64-bit | i64 | u64 | 9.223e18 |
| 128-bit | i128 | u128 | 1.701e38 |
| arch | isize | usize | depends on de computer |

- Floating-point numbers:<br>
f32 and f64

- Boolean:<br>
bool (true or false)

Data types:
tuples and arrays

conditions must be a bool



&#xa0;

<a href="#top">Back to top</a>
