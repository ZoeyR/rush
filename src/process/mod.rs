use std::process::Child;
use wait_timeout::ChildExt;



pub fn check_children(children: Vec<Child>) -> Vec<Child> {
    let mut ret: Vec<Child> = Vec::new();
    for mut child in children {
        match child.wait_timeout_ms(0).unwrap() {
            Some(status) => {
                println!("{}:{}", child.id(), status);
            },
            None => ret.push(child),
        }
    }

    ret
}
