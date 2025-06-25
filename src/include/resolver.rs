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
        path.ends_with(".json")
    }

    pub fn resolve_path(base_dir: &Path, relative_path: &str) -> Result<std::path::PathBuf> {
        let full_path = base_dir.join(relative_path);
        if !full_path.exists() {
            return Err(YincError::Include(format!("File not found: {}", full_path.display())));
        }
        Ok(full_path)
    }
}
