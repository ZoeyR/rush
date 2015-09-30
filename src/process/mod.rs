use std::process::Child;
use wait_timeout::ChildExt;

/// Background shell process
pub struct ShellProcess {
    pub name: String,
    pub child: Child,
}

pub fn check_children(processes: Vec<ShellProcess>) -> Vec<ShellProcess> {
    let mut ret: Vec<ShellProcess> = Vec::new();
    for mut process in processes {
        match process.child.wait_timeout_ms(0).unwrap() {
            Some(status) => {
                println!("{}: {}", process.name, status);
            },
            None => ret.push(process),
        }
    }

    ret
}
