use std::{fs, path::Path};

use crate::{
    framework::Framework,
    parser::{
        Config, Parser,
        formats::{FileFormat, json::JsonParser, yaml::YamlParser},
    },
};

pub struct ParserFactory {
    pub file_type: FileFormat,
    pub content: String,
    pub framework: Framework,
}

impl ParserFactory {
    pub fn new(config_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(&config_path)?;
        let ext = Path::new(config_path)
            .extension()
            .and_then(|s| s.to_str())
            .unwrap_or_default();

        let file_type = match ext {
            "json" => FileFormat::Json,
            "yaml" | "yml" => FileFormat::Yaml,
            _ => return Err(format!("Unsupported file format: {}", ext).into()),
        };

        Ok(Self {
            file_type,
            content,
            framework: Framework::default(),
        })
    }

    pub fn with_framework(
        framework: Framework,
        config_path: &str,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let mut factory = Self::new(config_path)?;
        factory.framework = framework;
        Ok(factory)
    }

    pub fn parse(&self) -> Result<Config, Box<dyn std::error::Error>> {
        match self.file_type {
            FileFormat::Json => JsonParser::parse(&self.content),
            FileFormat::Yaml => YamlParser::parse(&self.content),
        }
    }
}
