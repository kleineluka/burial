// imports
use std::process::{Command, Child, Stdio};
use std::io;
use std::result;

pub struct ProcessHandle {
    child: Option<Child>,
}

impl ProcessHandle {

    // create the process handler
    pub fn new() -> Self {
        ProcessHandle { child: None }
    }

    // start the process
    pub fn start_exe(&mut self, exe_path: &str) -> io::Result<()> {
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
    pub fn stop_process(&mut self) -> io::Result<()> {
        if let Some(child) = &mut self.child {
            child.kill()?;
            self.child = None;
        } else {
            println!("No process is running.");
        }
        Ok(())
    }

}

pub fn kill_process(process_name: &str) -> result::Result<(), String> {
    // on windows use taskkill
    #[cfg(target_os = "windows")]
    let output = Command::new("taskkill")
        .args(&["/F", "/IM", process_name])
        .output()
        .map_err(|e| format!("Failed to execute command: {}", e))?;

    // on unix-like systems (linux, macos), use pkill
    #[cfg(not(target_os = "windows"))]
    let output = Command::new("pkill")
        .arg(process_name)
        .output()
        .map_err(|e| format!("Failed to execute command: {}", e))?;

    if output.status.success() {
        Ok(())
    } else {
        println!("Failed to kill process: {}", String::from_utf8_lossy(&output.stderr));
        Ok(()) // keep going, assume it just wasn't running
    }

}