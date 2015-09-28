pub use self::builtin::*;
use std;
use std::process::{Command, Child};
mod builtin;

extern crate shlex;

pub struct ShellCommand {
    name: String,
    args: Vec<String>,
    background: bool,
}

impl ShellCommand {
    pub fn new(input: &str) -> Option<Self> {
        let args = shlex::split(input);

        args.and_then(|mut args| {
            if args.len() == 0 || (args.len() == 1 && args[0] == "&") {
                None
            } else {

                let bg = if args[args.len() - 1] == "&" {
                    let len = args.len();
                    args.remove(len - 1);
                    true
                } else {
                    false
                };

                let name = args.remove(0);
                Some(ShellCommand{name: name, background: bg, args: args})
            }
        })
    }
}

pub fn run_command(cmd: &ShellCommand) -> Result<Option<Child>, String> {
    let name: &str = &cmd.name;
    match name {
        CD => {
            cd(cmd);
        },
        PWD => {
            pwd(cmd);
        },
        EXIT => {
            std::process::exit(0);
        },
        _ => {
            return run_extern(cmd);
        }
    }
    return Ok(None);
}

fn run_extern(cmd: &ShellCommand) -> Result<Option<Child>, String>{
    let child = Command::new(&cmd.name)
        .args(&cmd.args)
        .spawn()
        .map_err(|e| format!("{}", e));

    match child {
        Ok(mut childproc) => {
            if !cmd.background {
                childproc.wait();
                Ok(None)
            } else {
                Ok(Some(childproc))
            }
        },
        Err(error) => {
            Err(error)
        }
    }
}
