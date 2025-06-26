//! YAML include processor
//!
//! This library provides functionality to process YAML files with `!include` directives,
//! allowing for modular YAML composition.

pub mod error;
pub mod include;
pub mod parser;
pub mod http;
pub mod shell;

pub use error::YincError;
pub use include::processor::IncludeProcessor;
pub use parser::yaml::YamlParser;

/// Result type used throughout the library
pub type Result<T> = std::result::Result<T, YincError>;

/// Configuration for the YAML include processor
#[derive(Debug, Clone)]
pub struct Config {
    /// Width of indentation (default: 2)
    pub indent_width: usize,
    /// Whether to output multiple documents
    pub output_multi_documents: bool,
    /// Tag used for include directives (default: "!include")
    pub include_tag: String,
    /// Tag used for replace directives (default: "!replace")
    pub replace_tag: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            indent_width: 2,
            output_multi_documents: false,
            include_tag: "!include".to_string(),
            replace_tag: "!replace".to_string(),
        }
    }
}
