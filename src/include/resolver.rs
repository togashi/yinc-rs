/// Resolver utilities for different include types
use crate::{Result, YincError};
use std::path::Path;

pub struct Resolver;

impl Resolver {
    pub fn is_url(path: &str) -> bool {
        path.starts_with("http://") || path.starts_with("https://")
    }

    pub fn is_glob_pattern(path: &str) -> bool {
        path.contains('*') || path.contains('?') || path.contains('[')
    }

    pub fn is_json_file(path: &str) -> bool {
        path.to_lowercase().ends_with(".json")
    }

    pub fn resolve_path(base_dir: &Path, relative_path: &str) -> Result<std::path::PathBuf> {
        let full_path = base_dir.join(relative_path);
        if !full_path.exists() {
            return Err(YincError::Include(format!("File not found: {}", full_path.display())));
        }
        Ok(full_path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_is_url() {
        assert!(Resolver::is_url("http://example.com/file.yml"));
        assert!(Resolver::is_url("https://example.com/file.yml"));
        assert!(!Resolver::is_url("ftp://example.com/file.yml"));
        assert!(!Resolver::is_url("file.yml"));
        assert!(!Resolver::is_url("/path/to/file.yml"));
        assert!(!Resolver::is_url(""));
    }

    #[test]
    fn test_is_glob_pattern() {
        assert!(Resolver::is_glob_pattern("*.yml"));
        assert!(Resolver::is_glob_pattern("config/*.yml"));
        assert!(Resolver::is_glob_pattern("file?.yml"));
        assert!(Resolver::is_glob_pattern("file[abc].yml"));
        assert!(Resolver::is_glob_pattern("**/*.yml"));
        assert!(!Resolver::is_glob_pattern("file.yml"));
        assert!(!Resolver::is_glob_pattern("/path/to/file.yml"));
        assert!(!Resolver::is_glob_pattern(""));
    }

    #[test]
    fn test_is_json_file() {
        assert!(Resolver::is_json_file("data.json"));
        assert!(Resolver::is_json_file("path/to/data.json"));
        assert!(Resolver::is_json_file("file.JSON"));
        assert!(!Resolver::is_json_file("data.yml"));
        assert!(!Resolver::is_json_file("data.json.txt"));
        assert!(!Resolver::is_json_file("json"));
        assert!(!Resolver::is_json_file(""));
    }

    #[test]
    fn test_resolve_path_existing_file() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.yml");
        fs::write(&file_path, "test content").unwrap();

        let resolved = Resolver::resolve_path(temp_dir.path(), "test.yml");
        assert!(resolved.is_ok());
        assert_eq!(resolved.unwrap(), file_path);
    }

    #[test]
    fn test_resolve_path_non_existing_file() {
        let temp_dir = TempDir::new().unwrap();
        
        let resolved = Resolver::resolve_path(temp_dir.path(), "non_existing.yml");
        assert!(resolved.is_err());
        
        match resolved.unwrap_err() {
            YincError::Include(msg) => assert!(msg.contains("File not found")),
            _ => panic!("Expected YincError::Include"),
        }
    }

    #[test]
    fn test_resolve_path_with_subdirectory() {
        let temp_dir = TempDir::new().unwrap();
        let sub_dir = temp_dir.path().join("configs");
        fs::create_dir(&sub_dir).unwrap();
        let file_path = sub_dir.join("test.yml");
        fs::write(&file_path, "test content").unwrap();

        let resolved = Resolver::resolve_path(temp_dir.path(), "configs/test.yml");
        assert!(resolved.is_ok());
        assert_eq!(resolved.unwrap(), file_path);
    }

    #[test]
    fn test_resolve_path_with_relative_path() {
        let temp_dir = TempDir::new().unwrap();
        let sub_dir = temp_dir.path().join("a").join("b");
        fs::create_dir_all(&sub_dir).unwrap();
        let file_path = temp_dir.path().join("test.yml");
        fs::write(&file_path, "test content").unwrap();

        let resolved = Resolver::resolve_path(&sub_dir, "../../test.yml");
        assert!(resolved.is_ok());
        assert_eq!(resolved.unwrap().canonicalize().unwrap(), file_path.canonicalize().unwrap());
    }

    #[test]
    fn test_edge_cases() {
        assert!(!Resolver::is_url("http:file.yml"));
        assert!(!Resolver::is_url("https:"));
        
        assert!(Resolver::is_glob_pattern("*"));
        assert!(Resolver::is_glob_pattern("\\*")); // Backslash doesn't escape in Rust glob patterns
        
        assert!(Resolver::is_json_file(".json"));
        assert!(!Resolver::is_json_file("json."));
    }
}
