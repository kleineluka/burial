// imports
use std::process::{Command, Child, Stdio};
use std::io::Result;

pub struct ProcessHandle {
    child: Option<Child>,
}

impl ProcessHandle {

    // create the process handler
    pub fn new() -> Self {
        ProcessHandle { child: None }
    }

    // start the process
    pub fn start_exe(&mut self, exe_path: &str) -> Result<()> {
        if self.child.is_some() {
            println!("Process is already running.");
            return Ok(());
        }
        let child = Command::new(exe_path)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()?;

        self.child = Some(child);
        Ok(())
    }

    // stop the process
    pub fn stop_process(&mut self) -> Result<()> {
        if let Some(child) = &mut self.child {
            child.kill()?;
            self.child = None;
        } else {
            println!("No process is running.");
        }
        Ok(())
    }

}