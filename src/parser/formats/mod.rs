use std::str::FromStr;

pub mod json;
pub mod yaml;

pub enum FileFormat {
    Json,
    Yaml,
}
impl FromStr for FileFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "json" => Ok(Self::Json),
            "yaml" | "yml" => Ok(Self::Yaml),
            _ => Err(format!("Unsupported file format: {}", s)),
        }
    }
}

