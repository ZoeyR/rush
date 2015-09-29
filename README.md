# RUSH
###### A unix shell written in [Rust](rust-lang.org).

Rush is a unix shell written in rust. The goal is to eventually have a fully featured shell that can be used for everyday use. To that end, .sh script compatibility is an end goal.

### Current features:
* Built-in commands
    * ```cd``` - change the current working directory.
    * ```pwd``` - print the current working directory.
* Execute external programs, both as blocking and in backgound

### Features planned soon:
* Parse commands to replace enironment variables with their values.
* Commands to get and set environment variables.

## Compiling rush
Rush requires the following programs to be installed on your system in order to compile:
* [rustc](rust-lang.org) - v1.3 stable
* [cargo](crates.io)
* git (optional)

#### Steps to compile
1. Get a copy of the source code  
```git clone https://github.com/dgriffen/rush```
2. Change into the source directory  
```cd rush```
3. Compile with cargo  
```cargo build --release```
4. Either run using ```cargo run --release``` or run the executable file directly. The binary will be located at $RUSH_HOME/target/release/rush.
