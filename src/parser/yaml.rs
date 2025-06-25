/// YAML parsing utilities
use crate::{Result, YincError};
use serde_yaml::Value;

pub struct YamlParser;

impl YamlParser {
    pub fn parse(content: &str) -> Result<Value> {
        serde_yaml::from_str(content).map_err(YincError::from)
    }

    pub fn to_string(value: &Value) -> Result<String> {
        serde_yaml::to_string(value).map_err(YincError::from)
    }

    pub fn parse_json_as_yaml(json_content: &str) -> Result<Value> {
        let json_value: serde_json::Value = serde_json::from_str(json_content)?;
        let yaml_str = serde_yaml::to_string(&json_value)?;
        let yaml_value: Value = serde_yaml::from_str(&yaml_str)?;
        Ok(yaml_value)
    }
}
