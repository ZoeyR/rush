pub use self::builtin::*;

use std;
use std::process::Command;
use process::ShellProcess;
use shell::Shell;
mod builtin;

extern crate shlex;

pub struct ShellCommand {
    pub name: String,
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

pub fn run_command(cmd: &ShellCommand, shell: &mut Shell) -> Result<(), String> {
    let name: &str = &cmd.name;
    match name {
        CD => {
            try!(cd(cmd));
        },
        PWD => {
            try!(pwd(cmd));
        },
        JOBS => {
            try!(jobs(cmd, shell));
        },
        EXIT => {
            std::process::exit(0);
        },
        _ => {
            match try!(run_extern(cmd)) {
                Some(child) => shell.jobs.push(child),
                None => {}
            }
        }
    }
    return Ok(());
}

fn run_extern(cmd: &ShellCommand) -> Result<Option<ShellProcess>, String>{
    let mut child = try!(Command::new(&cmd.name)
        .args(&cmd.args)
        .spawn()
        .map_err(|e| format!("{}", e)));

    if !cmd.background {
        try!(child.wait().map_err(|e| format!("{}", e)));
        Ok(None)
    } else {
        Ok(Some(ShellProcess{name: cmd.name.clone(), child: child}))
    }
}
