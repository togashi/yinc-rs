use std::fmt;

#[derive(Debug)]
pub enum YincError {
    Io(std::io::Error),
    Yaml(serde_yaml::Error),
    Json(serde_json::Error),
    Http(reqwest::Error),
    Glob(glob::PatternError),
    Shell(String),
    Include(String),
    Parse(String),
}

impl fmt::Display for YincError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            YincError::Io(err) => write!(f, "IO error: {}", err),
            YincError::Yaml(err) => write!(f, "YAML error: {}", err),
            YincError::Json(err) => write!(f, "JSON error: {}", err),
            YincError::Http(err) => write!(f, "HTTP error: {}", err),
            YincError::Glob(err) => write!(f, "Glob pattern error: {}", err),
            YincError::Shell(err) => write!(f, "Shell command error: {}", err),
            YincError::Include(err) => write!(f, "Include error: {}", err),
            YincError::Parse(err) => write!(f, "Parse error: {}", err),
        }
    }
}

impl std::error::Error for YincError {}

impl From<std::io::Error> for YincError {
    fn from(err: std::io::Error) -> Self {
        YincError::Io(err)
    }
}

impl From<serde_yaml::Error> for YincError {
    fn from(err: serde_yaml::Error) -> Self {
        YincError::Yaml(err)
    }
}

impl From<serde_json::Error> for YincError {
    fn from(err: serde_json::Error) -> Self {
        YincError::Json(err)
    }
}

impl From<reqwest::Error> for YincError {
    fn from(err: reqwest::Error) -> Self {
        YincError::Http(err)
    }
}

impl From<glob::PatternError> for YincError {
    fn from(err: glob::PatternError) -> Self {
        YincError::Glob(err)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_yinc_error_display() {
        let io_err = YincError::Io(std::io::Error::new(std::io::ErrorKind::NotFound, "file not found"));
        assert_eq!(io_err.to_string(), "IO error: file not found");

        let shell_err = YincError::Shell("command failed".to_string());
        assert_eq!(shell_err.to_string(), "Shell command error: command failed");

        let include_err = YincError::Include("invalid include directive".to_string());
        assert_eq!(include_err.to_string(), "Include error: invalid include directive");

        let parse_err = YincError::Parse("parsing failed".to_string());
        assert_eq!(parse_err.to_string(), "Parse error: parsing failed");
    }

    #[test]
    fn test_yinc_error_from_io_error() {
        let io_error = std::io::Error::new(std::io::ErrorKind::PermissionDenied, "access denied");
        let yinc_error: YincError = io_error.into();
        
        match yinc_error {
            YincError::Io(_) => {},
            _ => panic!("Expected YincError::Io"),
        }
    }

    #[test]
    fn test_yinc_error_from_yaml_error() {
        let yaml_str = "invalid: yaml: content:";
        let yaml_error = serde_yaml::from_str::<serde_yaml::Value>(yaml_str).unwrap_err();
        let yinc_error: YincError = yaml_error.into();
        
        match yinc_error {
            YincError::Yaml(_) => {},
            _ => panic!("Expected YincError::Yaml"),
        }
    }

    #[test]
    fn test_yinc_error_from_json_error() {
        let json_str = "invalid json";
        let json_error = serde_json::from_str::<serde_json::Value>(json_str).unwrap_err();
        let yinc_error: YincError = json_error.into();
        
        match yinc_error {
            YincError::Json(_) => {},
            _ => panic!("Expected YincError::Json"),
        }
    }

    #[test]
    fn test_yinc_error_from_glob_error() {
        let glob_error = glob::Pattern::new("[").unwrap_err();
        let yinc_error: YincError = glob_error.into();
        
        match yinc_error {
            YincError::Glob(_) => {},
            _ => panic!("Expected YincError::Glob"),
        }
    }

    #[test]
    fn test_yinc_error_is_std_error() {
        let err = YincError::Shell("test".to_string());
        let _: &dyn std::error::Error = &err;
    }

    #[test]
    fn test_yinc_error_debug() {
        let err = YincError::Parse("test error".to_string());
        let debug_str = format!("{:?}", err);
        assert!(debug_str.contains("Parse"));
        assert!(debug_str.contains("test error"));
    }
}
