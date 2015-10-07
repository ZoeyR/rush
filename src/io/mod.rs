use std::io::{self, Write};
use std::env;

pub fn get_prompt() -> String {
    let dir = env::current_dir().unwrap();
    format!("{} >", dir.display())
}

pub fn get_input() -> io::Result<String> {
    print!("{}", get_prompt());
    try!(io::stdout().flush());

    let mut buffer = String::new();
    try!(io::stdin().read_line(&mut buffer));

    Ok(buffer)
}
