use std::io::{self, Write};
use std::env;
use std::process::Child;
use cmd::ShellCommand;

pub mod cmd;
mod process;

extern crate wait_timeout;

fn main() {
    let mut input = String::new();
    let mut children: Vec<Child> = Vec::new();
    loop {
        children = process::check_children(children);
        // if we aren't in a valid directory I will eat my hat.
        let dir = env::current_dir().unwrap();
        print!("{} >", dir.display());
        io::stdout().flush();

        input.clear();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {},
            Err(error) => {
                println!("Error getting input: {}", error);
                continue;
            }
        }

        let cmd = ShellCommand::new(&input);
        match cmd {
            Some(cmd) => {
                match cmd::run_command(&cmd) {
                    Ok(option) => {
                        match option {
                            Some(child) => children.push(child),
                            None => {},
                        }
                    },
                    Err(error) => {
                        println!("Error executing command: {}", error);
                        continue;
                    }
                }
            },
            None => {
                continue;
            }
        }
    }
}
