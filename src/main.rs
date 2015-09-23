use std::io::{self, Write};
use std::env;
use cmd::ShellCommand;

pub mod cmd;

fn main() {
    let mut input = String::new();
    loop {
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
