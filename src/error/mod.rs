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
