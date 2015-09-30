use std::env;
use std::path::Path;

pub const EXIT: &'static str = "exit";
pub const CD: &'static str = "cd";
pub const PWD: &'static str = "pwd";
pub const JOBS: &'static str = "jobs";

pub fn cd(cmd: &super::ShellCommand) -> Result<(), String> {
    let dir = match cmd.args.len() {
        0 => env::home_dir(),
        1 => Some(Path::new(&cmd.args[0]).to_path_buf()),
        _ => return Err("Too many arguments!".to_string())
    };

    if dir.is_none() {
        return Err("No directory to change to".to_string());
    }


    env::set_current_dir(dir.unwrap()).map_err(|e| format!("{}", e))
}

pub fn pwd(cmd: &super::ShellCommand) -> Result<(), String> {
    if cmd.args.len() > 0  {
        return Err("Too many arguments!".to_string());
    }

    println!("{}", env::current_dir().unwrap().to_string_lossy());
    Ok(())
}

pub fn jobs(_: &super::ShellCommand, shell: &::shell::Shell) -> Result<(), String> {
    for job in &shell.jobs {
        println!("{}", job.name);
    }
    Ok(())
}
