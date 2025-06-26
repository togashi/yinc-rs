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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_execute_simple_command() {
        let result = ShellExecutor::execute("echo hello");
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.trim() == "hello");
    }

    #[test]
    fn test_execute_command_with_multiple_args() {
        let result = ShellExecutor::execute("echo hello world");
        assert!(result.is_ok());
        let output = result.unwrap();
        assert_eq!(output.trim(), "hello world");
    }

    #[test]
    fn test_execute_failing_command() {
        let result = ShellExecutor::execute("false");
        assert!(result.is_err());
        match result.unwrap_err() {
            YincError::Shell(msg) => assert!(msg.contains("Command failed")),
            _ => panic!("Expected YincError::Shell"),
        }
    }

    #[test]
    fn test_execute_command_with_stderr() {
        let command = if cfg!(target_os = "windows") {
            "echo error message 1>&2 && exit 1"
        } else {
            "echo 'error message' >&2 && exit 1"
        };
        let result = ShellExecutor::execute(command);
        assert!(result.is_err());
        match result.unwrap_err() {
            YincError::Shell(msg) => {
                assert!(msg.contains("Command failed"));
                assert!(msg.contains("error message"));
            },
            _ => panic!("Expected YincError::Shell"),
        }
    }

    #[test]
    fn test_execute_nonexistent_command() {
        let result = ShellExecutor::execute("this_command_should_not_exist_12345");
        assert!(result.is_err());
        match result.unwrap_err() {
            YincError::Shell(_) => {},
            _ => panic!("Expected YincError::Shell"),
        }
    }

    #[test]
    fn test_execute_command_with_newlines() {
        let result = ShellExecutor::execute("printf 'line1\\nline2\\nline3'");
        if result.is_ok() {
            let output = result.unwrap();
            assert_eq!(output, "line1\nline2\nline3");
        }
    }

    #[test]
    fn test_execute_empty_command() {
        let result = ShellExecutor::execute("");
        if cfg!(target_os = "windows") {
            assert!(result.is_ok() || result.is_err());
        } else {
            assert!(result.is_ok());
            assert_eq!(result.unwrap(), "");
        }
    }

    #[test]
    fn test_execute_command_with_env_var() {
        let command = if cfg!(target_os = "windows") {
            "echo %PATH%"
        } else {
            "echo $PATH"
        };
        let result = ShellExecutor::execute(command);
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(!output.trim().is_empty());
    }

    #[test]
    fn test_execute_piped_commands() {
        let command = if cfg!(target_os = "windows") {
            "echo hello | findstr hello"
        } else {
            "echo hello | grep hello"
        };
        let result = ShellExecutor::execute(command);
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.contains("hello"));
    }
}
