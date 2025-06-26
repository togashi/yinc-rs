/// Directive types and parsing logic
#[derive(Debug, Clone)]
pub enum Directive {
    Include(String),
    Shell(String),
    Json(String),
}

impl Directive {
    pub fn parse(input: &str) -> Option<Self> {
        if input.starts_with("$(") && input.ends_with(")") {
            let inner = &input[2..input.len() - 1];
            if inner.starts_with("shell ") {
                return Some(Directive::Shell(inner[6..].to_string()));
            }
            if inner.starts_with("json ") {
                return Some(Directive::Json(inner[5..].to_string()));
            }
        } else {
            return Some(Directive::Include(input.to_string()));
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_include_directive() {
        let input = "path/to/file.yml";
        let directive = Directive::parse(input);
        assert!(directive.is_some());
        
        match directive.unwrap() {
            Directive::Include(path) => assert_eq!(path, "path/to/file.yml"),
            _ => panic!("Expected Include directive"),
        }
    }

    #[test]
    fn test_parse_shell_directive() {
        let input = "$(shell echo hello)";
        let directive = Directive::parse(input);
        assert!(directive.is_some());
        
        match directive.unwrap() {
            Directive::Shell(cmd) => assert_eq!(cmd, "echo hello"),
            _ => panic!("Expected Shell directive"),
        }
    }

    #[test]
    fn test_parse_json_directive() {
        let input = "$(json path/to/data.json)";
        let directive = Directive::parse(input);
        assert!(directive.is_some());
        
        match directive.unwrap() {
            Directive::Json(path) => assert_eq!(path, "path/to/data.json"),
            _ => panic!("Expected Json directive"),
        }
    }

    #[test]
    fn test_parse_empty_shell_command() {
        let input = "$(shell )";
        let directive = Directive::parse(input);
        assert!(directive.is_some());
        
        match directive.unwrap() {
            Directive::Shell(cmd) => assert_eq!(cmd, ""),
            _ => panic!("Expected Shell directive"),
        }
    }

    #[test]
    fn test_parse_shell_with_complex_command() {
        let input = "$(shell ls -la | grep .yml | head -5)";
        let directive = Directive::parse(input);
        assert!(directive.is_some());
        
        match directive.unwrap() {
            Directive::Shell(cmd) => assert_eq!(cmd, "ls -la | grep .yml | head -5"),
            _ => panic!("Expected Shell directive"),
        }
    }

    #[test]
    fn test_parse_include_with_glob_pattern() {
        let input = "configs/*.yml";
        let directive = Directive::parse(input);
        assert!(directive.is_some());
        
        match directive.unwrap() {
            Directive::Include(path) => assert_eq!(path, "configs/*.yml"),
            _ => panic!("Expected Include directive"),
        }
    }

    #[test]
    fn test_parse_include_with_url() {
        let input = "https://example.com/config.yml";
        let directive = Directive::parse(input);
        assert!(directive.is_some());
        
        match directive.unwrap() {
            Directive::Include(path) => assert_eq!(path, "https://example.com/config.yml"),
            _ => panic!("Expected Include directive"),
        }
    }

    #[test]
    fn test_parse_malformed_directive() {
        let input = "$(invalid directive)";
        let directive = Directive::parse(input);
        assert!(directive.is_none());
    }

    #[test]
    fn test_parse_unclosed_directive() {
        let input = "$(shell echo hello";
        let directive = Directive::parse(input);
        assert!(directive.is_some());
        
        match directive.unwrap() {
            Directive::Include(path) => assert_eq!(path, "$(shell echo hello"),
            _ => panic!("Expected Include directive for malformed input"),
        }
    }

    #[test]
    fn test_directive_clone() {
        let directive = Directive::Shell("echo test".to_string());
        let cloned = directive.clone();
        
        match cloned {
            Directive::Shell(cmd) => assert_eq!(cmd, "echo test"),
            _ => panic!("Clone failed"),
        }
    }

    #[test]
    fn test_directive_debug() {
        let directive = Directive::Include("test.yml".to_string());
        let debug_str = format!("{:?}", directive);
        assert!(debug_str.contains("Include"));
        assert!(debug_str.contains("test.yml"));
    }
}
