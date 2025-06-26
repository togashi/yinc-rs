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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_valid_yaml() {
        let yaml_content = r#"
        key: value
        number: 42
        array:
          - item1
          - item2
        nested:
          inner: content
        "#;
        
        let result = YamlParser::parse(yaml_content);
        assert!(result.is_ok());
        
        let value = result.unwrap();
        assert_eq!(value["key"], "value");
        assert_eq!(value["number"], 42);
        assert_eq!(value["array"][0], "item1");
        assert_eq!(value["array"][1], "item2");
        assert_eq!(value["nested"]["inner"], "content");
    }

    #[test]
    fn test_parse_invalid_yaml() {
        let invalid_yaml = "invalid: yaml: content:";
        let result = YamlParser::parse(invalid_yaml);
        assert!(result.is_err());
        
        match result.unwrap_err() {
            YincError::Yaml(_) => {},
            _ => panic!("Expected YincError::Yaml"),
        }
    }

    #[test]
    fn test_to_string() {
        let mut map = serde_yaml::Mapping::new();
        map.insert(Value::String("key".to_string()), Value::String("value".to_string()));
        map.insert(Value::String("number".to_string()), Value::Number(42.into()));
        
        let value = Value::Mapping(map);
        let result = YamlParser::to_string(&value);
        assert!(result.is_ok());
        
        let yaml_str = result.unwrap();
        assert!(yaml_str.contains("key: value"));
        assert!(yaml_str.contains("number: 42"));
    }

    #[test]
    fn test_parse_json_as_yaml() {
        let json_content = r#"{
            "name": "test",
            "version": 1,
            "features": ["feature1", "feature2"],
            "config": {
                "enabled": true,
                "timeout": 30
            }
        }"#;
        
        let result = YamlParser::parse_json_as_yaml(json_content);
        assert!(result.is_ok());
        
        let value = result.unwrap();
        assert_eq!(value["name"], "test");
        assert_eq!(value["version"], 1);
        assert_eq!(value["features"][0], "feature1");
        assert_eq!(value["features"][1], "feature2");
        assert_eq!(value["config"]["enabled"], true);
        assert_eq!(value["config"]["timeout"], 30);
    }

    #[test]
    fn test_parse_json_as_yaml_invalid_json() {
        let invalid_json = "{ invalid json }";
        let result = YamlParser::parse_json_as_yaml(invalid_json);
        assert!(result.is_err());
        
        match result.unwrap_err() {
            YincError::Json(_) => {},
            _ => panic!("Expected YincError::Json"),
        }
    }

    #[test]
    fn test_parse_empty_yaml() {
        let empty_yaml = "";
        let result = YamlParser::parse(empty_yaml);
        assert!(result.is_ok());
        
        let value = result.unwrap();
        assert!(value.is_null());
    }

    #[test]
    fn test_parse_yaml_with_special_types() {
        let yaml_content = r#"
        string: "quoted string"
        boolean: true
        null_value: null
        float: 3.14
        date: 2023-01-01
        "#;
        
        let result = YamlParser::parse(yaml_content);
        assert!(result.is_ok());
        
        let value = result.unwrap();
        assert_eq!(value["string"], "quoted string");
        assert_eq!(value["boolean"], true);
        assert!(value["null_value"].is_null());
        assert_eq!(value["float"], 3.14);
        assert_eq!(value["date"], "2023-01-01");
    }
}
