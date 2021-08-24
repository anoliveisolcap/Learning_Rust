  &#xa0;

</div>

<h1 align="center"> 🦀 Learning Rust ⚙ </h1>

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

Project to learn rust

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

&#xa0;

<a href="#top">Back to top</a>
