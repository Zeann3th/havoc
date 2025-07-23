#![allow(dead_code)]
use std::{fs, path::Path};

use serde::{Deserialize, Serialize};

pub mod json;
pub mod yaml;

pub trait Parser {
    fn parse(input: &str) -> Result<Config, Box<dyn std::error::Error>>;
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub metadata: Metadata,
    pub spec: Spec,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Metadata {
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub author: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Spec {
    pub host: String,
    pub port: u16,
    pub services: Vec<Service>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Service {
    pub name: String,
    pub proto: String,
    pub url: String,
    pub endpoints: Vec<Endpoint>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Endpoint {
    pub rpc: String,
    pub method: String,
    pub path: String,
    pub request: Request,
    pub response: Response,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Request {
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(default)]
    pub fields: Vec<Field>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Response {
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(default)]
    pub fields: Vec<Field>,
    #[serde(default)]
    pub cookies: Vec<Cookie>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Field {
    pub name: String,
    #[serde(rename = "type")]
    pub type_: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Cookie {
    pub name: String,
    pub options: Option<CookieOptions>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CookieOptions {
    #[serde(rename = "httpOnly")]
    pub http_only: Option<bool>,
    pub secure: Option<bool>,
    #[serde(rename = "sameSite")]
    pub same_site: Option<String>,
    #[serde(rename = "maxAge")]
    pub max_age: Option<usize>,
    pub path: Option<String>,
    pub domain: Option<String>,
    pub partitioned: Option<bool>,
}

pub enum FileType {
    Json,
    Yaml,
}

pub struct ParserFactory {
    pub file_type: FileType,
    pub content: String,
}

impl ParserFactory {
    pub fn new(config_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(&config_path)?;
        let ext = Path::new(config_path)
            .extension()
            .and_then(|s| s.to_str())
            .unwrap_or_default();

        let file_type = match ext {
            "json" => FileType::Json,
            "yaml" | "yml" => FileType::Yaml,
            _ => return Err(format!("Unsupported file format: {}", ext).into()),
        };

        Ok(Self { file_type, content })
    }

    pub fn parse(&self) -> Result<Config, Box<dyn std::error::Error>> {
        match self.file_type {
            FileType::Json => json::JsonParser::parse(&self.content),
            FileType::Yaml => yaml::YamlParser::parse(&self.content),
        }
    }
}
