use ::process::ShellProcess;

pub struct Shell {
    pub jobs: Vec<ShellProcess>,
}

impl Shell {
    pub fn new() -> Self {
        Shell {jobs: Vec::new()}
    }
}
