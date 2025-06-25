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
