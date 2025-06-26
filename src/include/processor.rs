use crate::{Config, Result, YincError};
use serde_yaml::Value;
use std::path::Path;
use std::collections::HashMap;

/// Main processor for handling YAML include directives
pub struct IncludeProcessor {
    config: Config,
    client: reqwest::Client,
}

impl IncludeProcessor {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            client: reqwest::Client::new(),
        }
    }

    pub async fn process_file(&self, file_path: &Path) -> Result<String> {
        let content = std::fs::read_to_string(file_path)?;
        let mut value: Value = serde_yaml::from_str(&content)?;
        
        let base_dir = file_path.parent().unwrap_or(Path::new("."));
        self.process_value(&mut value, base_dir).await?;
        
        let result = serde_yaml::to_string(&value)?;
        Ok(result)
    }

    fn process_value<'a>(&'a self, value: &'a mut Value, base_dir: &'a Path) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<()>> + 'a>> {
        Box::pin(async move {
            match value {
                Value::Mapping(map) => {
                    let mut updates = HashMap::new();
                    
                    for (key, val) in map.iter() {
                        if let Some(include_path) = self.extract_include_directive(val) {
                            let resolved_value = self.resolve_include(&include_path, base_dir).await?;
                            updates.insert(key.clone(), resolved_value);
                        }
                    }
                    
                    // Apply updates
                    for (key, new_val) in updates {
                        map.insert(key, new_val);
                    }
                    
                    // Recursively process remaining values
                    for (_, val) in map.iter_mut() {
                        self.process_value(val, base_dir).await?;
                    }
                }
                Value::Sequence(seq) => {
                    for item in seq.iter_mut() {
                        self.process_value(item, base_dir).await?;
                    }
                }
                _ => {}
            }
            Ok(())
        })
    }

    fn extract_include_directive(&self, value: &Value) -> Option<String> {
        if let Value::Tagged(tagged) = value {
            if tagged.tag.to_string() == self.config.include_tag {
                if let Value::String(path) = &tagged.value {
                    return Some(path.clone());
                }
            }
        }
        None
    }

    async fn resolve_include(&self, include_path: &str, base_dir: &Path) -> Result<Value> {
        // Handle special directives like $(shell ...) and $(json ...)
        if include_path.starts_with("$(") && include_path.ends_with(")") {
            return self.resolve_special_directive(include_path, base_dir).await;
        }

        // Handle HTTP/HTTPS URLs
        if include_path.starts_with("http://") || include_path.starts_with("https://") {
            return self.resolve_http_include(include_path).await;
        }

        // Handle glob patterns
        if include_path.contains('*') {
            return self.resolve_glob_include(include_path, base_dir).await;
        }

        // Handle regular file includes
        self.resolve_file_include(include_path, base_dir).await
    }

    async fn resolve_special_directive(&self, directive: &str, base_dir: &Path) -> Result<Value> {
        let inner = &directive[2..directive.len() - 1]; // Remove $( and )
        
        if inner.starts_with("shell ") {
            let command = &inner[6..]; // Remove "shell "
            return self.resolve_shell_command(command, base_dir).await;
        }
        
        if inner.starts_with("json ") {
            let path = &inner[5..]; // Remove "json "
            return self.resolve_json_include(path, base_dir).await;
        }
        
        Err(YincError::Parse(format!("Unknown directive: {}", directive)))
    }

    async fn resolve_http_include(&self, url: &str) -> Result<Value> {
        let response = self.client.get(url).send().await?;
        let content = response.text().await?;
        let value: Value = serde_yaml::from_str(&content)?;
        Ok(value)
    }

    async fn resolve_glob_include(&self, pattern: &str, base_dir: &Path) -> Result<Value> {
        let full_pattern = base_dir.join(pattern);
        let pattern_str = full_pattern.to_string_lossy();
        
        let paths = glob::glob(&pattern_str)?;
        let mut results = Vec::new();
        
        for path in paths {
            let path = path.map_err(|e| YincError::Include(e.to_string()))?;
            let value = self.resolve_file_include(
                &path.strip_prefix(base_dir).unwrap_or(&path).to_string_lossy(),
                base_dir
            ).await?;
            results.push(value);
        }
        
        Ok(Value::Sequence(results))
    }

    async fn resolve_file_include(&self, file_path: &str, base_dir: &Path) -> Result<Value> {
        let full_path = base_dir.join(file_path);
        let content = std::fs::read_to_string(&full_path)?;
        
        if file_path.ends_with(".json") {
            let json_value: serde_json::Value = serde_json::from_str(&content)?;
            let yaml_str = serde_yaml::to_string(&json_value)?;
            let yaml_value: Value = serde_yaml::from_str(&yaml_str)?;
            Ok(yaml_value)
        } else {
            let value: Value = serde_yaml::from_str(&content)?;
            Ok(value)
        }
    }

    async fn resolve_json_include(&self, file_path: &str, base_dir: &Path) -> Result<Value> {
        let full_path = base_dir.join(file_path);
        let content = std::fs::read_to_string(&full_path)?;
        let json_value: serde_json::Value = serde_json::from_str(&content)?;
        let yaml_str = serde_yaml::to_string(&json_value)?;
        let yaml_value: Value = serde_yaml::from_str(&yaml_str)?;
        Ok(yaml_value)
    }

    async fn resolve_shell_command(&self, command: &str, _base_dir: &Path) -> Result<Value> {
        use std::process::Command;
        
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
        let value: Value = serde_yaml::from_str(&stdout)?;
        Ok(value)
    }
}
