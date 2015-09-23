pub use self::builtin::*;
use std;
use std::process::Command;
mod builtin;

extern crate shlex;

pub struct ShellCommand {
    name: String,
    args: Vec<String>,
    full_path: Option<String>,
}

impl ShellCommand {
    pub fn new(input: &str) -> Option<Self> {
        let mut args = match shlex::split(input) {
            Some(args) => {
                args
            },
            None => {
                return None;
            }
        };

        if args.len() == 0 {
            None
        } else {
            let name = args.remove(0);
            Some(ShellCommand{name: name, full_path: Option::None, args: args})
        }
    }
}

pub fn run_command(cmd: &ShellCommand) -> Result<(), String> {
    let name: &str = &cmd.name;
    match name {
        CD => {
            cd(cmd)
        },
        PWD => {
            pwd(cmd)
        },
        EXIT => {
            std::process::exit(0);
        },
        _ => {
            run_extern(cmd)
        }
    }
}

fn run_extern(cmd: &ShellCommand) -> Result<(), String>{
    let mut child = Command::new(&cmd.name)
        .args(&cmd.args)
        .spawn()
        .map_err(|e| format!("{}", e));

    match child {
        Ok(ref mut child) => {
            child.wait();
            Ok(())
        },
        Err(error) => {
            Err(error)
        }
    }
}
