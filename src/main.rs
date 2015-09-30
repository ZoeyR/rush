use std::io::{self, Write};
use std::env;
use cmd::ShellCommand;
use shell::Shell;

pub mod cmd;
mod process;
pub mod shell;

extern crate wait_timeout;

fn main() {
    let mut input = String::new();
    let mut shell = Shell::new();
    loop {
        shell.jobs = process::check_children(shell.jobs);
        // if we aren't in a valid directory I will eat my hat.
        let dir = env::current_dir().unwrap();
        print!("{} >", dir.display());
        io::stdout().flush();

        //get a line from the user
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
                match cmd::run_command(&cmd, &mut shell) {
                    Ok(_) => {},
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
