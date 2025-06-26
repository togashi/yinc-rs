use yinc::{Config, IncludeProcessor};
use std::fs;
use tempfile::TempDir;

#[tokio::test]
async fn test_basic_include() {
    let temp_dir = TempDir::new().unwrap();
    let base_path = temp_dir.path();
    
    // Create parent YAML file
    let parent_content = r#"
common:
  settings: !include child.yml
data:
  value: 42
"#;
    fs::write(base_path.join("parent.yml"), parent_content).unwrap();
    
    // Create child YAML file
    let child_content = r#"
debug: true
timeout: 30
features:
  - feature1
  - feature2
"#;
    fs::write(base_path.join("child.yml"), child_content).unwrap();
    
    // Process the parent file
    let config = Config::default();
    let processor = IncludeProcessor::new(config);
    let result = processor.process_file(&base_path.join("parent.yml")).await;
    
    assert!(result.is_ok());
    let output = result.unwrap();
    
    // Verify the included content is present
    assert!(output.contains("debug: true"));
    assert!(output.contains("timeout: 30"));
    assert!(output.contains("feature1"));
    assert!(output.contains("feature2"));
    assert!(output.contains("value: 42"));
}

#[tokio::test]
async fn test_json_include() {
    let temp_dir = TempDir::new().unwrap();
    let base_path = temp_dir.path();
    
    // Create parent YAML file with JSON include
    let parent_content = r#"
configuration: !include $(json config.json)
name: test-app
"#;
    fs::write(base_path.join("parent.yml"), parent_content).unwrap();
    
    // Create JSON file
    let json_content = r#"{
  "version": "1.0.0",
  "enabled": true,
  "settings": {
    "max_connections": 100,
    "retry_count": 3
  }
}"#;
    fs::write(base_path.join("config.json"), json_content).unwrap();
    
    // Process the parent file
    let config = Config::default();
    let processor = IncludeProcessor::new(config);
    let result = processor.process_file(&base_path.join("parent.yml")).await;
    
    assert!(result.is_ok());
    let output = result.unwrap();
    
    // Verify JSON content was converted to YAML and included
    assert!(output.contains("version"));
    assert!(output.contains("1.0.0"));
    assert!(output.contains("enabled: true"));
    assert!(output.contains("max_connections: 100"));
    assert!(output.contains("retry_count: 3"));
    assert!(output.contains("name: test-app"));
}

#[tokio::test]
async fn test_shell_command() {
    let temp_dir = TempDir::new().unwrap();
    let base_path = temp_dir.path();
    
    // Create parent YAML file with shell command
    let parent_content = r#"
timestamp: !include $(shell echo "2024-01-01")
hostname: !include $(shell echo "test-host")
data:
  key: value
"#;
    fs::write(base_path.join("parent.yml"), parent_content).unwrap();
    
    // Process the parent file
    let config = Config::default();
    let processor = IncludeProcessor::new(config);
    let result = processor.process_file(&base_path.join("parent.yml")).await;
    
    assert!(result.is_ok());
    let output = result.unwrap();
    
    // Verify shell command output was included
    assert!(output.contains("timestamp: 2024-01-01") || output.contains("timestamp: \"2024-01-01\""));
    assert!(output.contains("hostname: test-host") || output.contains("hostname: \"test-host\""));
    assert!(output.contains("key: value"));
}

#[tokio::test]
async fn test_glob_pattern_include() {
    let temp_dir = TempDir::new().unwrap();
    let base_path = temp_dir.path();
    let configs_dir = base_path.join("configs");
    fs::create_dir(&configs_dir).unwrap();
    
    // Create parent YAML file with glob pattern
    let parent_content = r#"
services: !include configs/*.yml
"#;
    fs::write(base_path.join("parent.yml"), parent_content).unwrap();
    
    // Create multiple config files
    let service1_content = r#"
- name: service1
  port: 8080
  enabled: true
"#;
    fs::write(configs_dir.join("service1.yml"), service1_content).unwrap();
    
    let service2_content = r#"
- name: service2
  port: 8081
  enabled: false
"#;
    fs::write(configs_dir.join("service2.yml"), service2_content).unwrap();
    
    // Process the parent file
    let config = Config::default();
    let processor = IncludeProcessor::new(config);
    let result = processor.process_file(&base_path.join("parent.yml")).await;
    
    assert!(result.is_ok());
    let output = result.unwrap();
    
    // Verify both services were included
    assert!(output.contains("service1"));
    assert!(output.contains("port: 8080"));
    assert!(output.contains("service2"));
    assert!(output.contains("port: 8081"));
}

#[tokio::test]
async fn test_nested_includes() {
    let temp_dir = TempDir::new().unwrap();
    let base_path = temp_dir.path();
    
    // Create parent YAML file
    let parent_content = r#"
app:
  config: !include level1.yml
"#;
    fs::write(base_path.join("parent.yml"), parent_content).unwrap();
    
    // Create first level include
    let level1_content = r#"
database: !include level2.yml
cache:
  enabled: true
"#;
    fs::write(base_path.join("level1.yml"), level1_content).unwrap();
    
    // Create second level include
    let level2_content = r#"
host: localhost
port: 5432
name: testdb
"#;
    fs::write(base_path.join("level2.yml"), level2_content).unwrap();
    
    // Process the parent file
    let config = Config::default();
    let processor = IncludeProcessor::new(config);
    let result = processor.process_file(&base_path.join("parent.yml")).await;
    
    assert!(result.is_ok());
    let output = result.unwrap();
    
    // Verify nested includes were processed
    assert!(output.contains("host: localhost"));
    assert!(output.contains("port: 5432"));
    assert!(output.contains("name: testdb"));
    assert!(output.contains("enabled: true"));
}

#[tokio::test]
async fn test_error_handling_missing_file() {
    let temp_dir = TempDir::new().unwrap();
    let base_path = temp_dir.path();
    
    // Create parent YAML file with non-existent include
    let parent_content = r#"
data: !include non_existent.yml
"#;
    fs::write(base_path.join("parent.yml"), parent_content).unwrap();
    
    // Process the parent file
    let config = Config::default();
    let processor = IncludeProcessor::new(config);
    let result = processor.process_file(&base_path.join("parent.yml")).await;
    
    // Should fail with file not found error
    assert!(result.is_err());
    let error = result.unwrap_err();
    assert!(error.to_string().contains("not found") || error.to_string().contains("No such file"));
}

#[tokio::test]
async fn test_simple_yaml_processing() {
    let temp_dir = TempDir::new().unwrap();
    let base_path = temp_dir.path();
    
    // Create simple YAML file without includes
    let input_content = r#"
test: data
value: 123
list:
  - item1
  - item2
"#;
    fs::write(base_path.join("simple.yml"), input_content).unwrap();
    
    // Process the file
    let config = Config::default();
    let processor = IncludeProcessor::new(config);
    let result = processor.process_file(&base_path.join("simple.yml")).await;
    
    assert!(result.is_ok());
    let output = result.unwrap();
    
    // Verify content is preserved
    assert!(output.contains("test: data"));
    assert!(output.contains("value: 123"));
    assert!(output.contains("item1"));
    assert!(output.contains("item2"));
}

#[tokio::test]
async fn test_config_indent_width() {
    let temp_dir = TempDir::new().unwrap();
    let base_path = temp_dir.path();
    
    // Create parent YAML file
    let parent_content = r#"
root:
  nested:
    data: !include data.yml
"#;
    fs::write(base_path.join("parent.yml"), parent_content).unwrap();
    
    // Create data file  
    let data_content = r#"
key1: value1
key2:
  - item1
  - item2
"#;
    fs::write(base_path.join("data.yml"), data_content).unwrap();
    
    // Process with custom indent width
    let mut config = Config::default();
    config.indent_width = 4;
    
    let processor = IncludeProcessor::new(config);
    let result = processor.process_file(&base_path.join("parent.yml")).await;
    
    assert!(result.is_ok());
    let output = result.unwrap();
    
    // Verify included content is present
    assert!(output.contains("key1: value1"));
    assert!(output.contains("key2:"));
    assert!(output.contains("item1"));
    assert!(output.contains("item2"));
}