/// Shell command executor
use crate::{Result, YincError};
use std::process::Command;

pub struct ShellExecutor;

impl ShellExecutor {
    pub fn execute(command: &str) -> Result<String> {
        let output = if cfg!(target_os = "windows") {
            Command::new("cmd")
                .args(["/C", command])
                .output()?
        } else {
            Command::new("sh")
                .arg("-c")
                .arg(command)
                .output()?
        };
        
        if !output.status.success() {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            return Err(YincError::Shell(format!("Command failed: {}", error_msg)));
        }
        
        let stdout = String::from_utf8_lossy(&output.stdout);
        Ok(stdout.to_string())
    }
}
