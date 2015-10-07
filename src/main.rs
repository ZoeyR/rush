use std::thread;
use std::io::Write;

use cmd::ShellCommand;

use shell::Shell;

use chan_signal::Signal;

pub mod cmd;
mod process;
mod io;
pub mod shell;

extern crate wait_timeout;
extern crate chan_signal;

fn main() {
    // create a thread to handle signals from the OS, this essentially creates a double thread,
    // which is useful to have a non-blocking check on signals
    let signal = chan_signal::notify(&[Signal::INT]);
    thread::spawn(move || {
        loop {
            let _ = signal.recv();
            sigint_handle();
        }
    });

    let mut shell = Shell::new();

    loop {

        shell.jobs = process::check_children(shell.jobs);

        // handle user input and any errors
        let input = match io::get_input() {
            Ok(result) => result,
            Err(error) => {
                println!("Error getting input: {}", error);
                continue;
            }
        };

        // create the command object from the input
        if let Some(cmd) = ShellCommand::new(&input) {
            match cmd::run_command(&cmd, &mut shell) {
                Ok(_) => {},
                Err(error) => {
                    println!("Error executing command {}: {}", cmd.name, error);
                    continue;
                }
            }
        }
    }
}

fn sigint_handle() {
    print!("\n");
    match std::io::stdout().flush() {
        Ok(_) => {},
        Err(error) => panic!("Failed to flush stdout: {}", error)
    }
}
