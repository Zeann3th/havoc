#![allow(dead_code)]
use serde::Deserialize;

pub mod json;
pub mod yaml;

pub trait Parser {
    fn parse(input: &str) -> Result<Config, Box<dyn std::error::Error>>;
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub metadata: Metadata,
    pub spec: Spec,
}

#[derive(Debug, Deserialize)]
pub struct Metadata {
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub author: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Spec {
    pub host: String,
    pub port: u16,
    pub services: Vec<Service>,
}

#[derive(Debug, Deserialize)]
pub struct Service {
    pub name: String,
    pub proto: String,
    pub url: String,
    pub endpoints: Vec<Endpoint>,
}

#[derive(Debug, Deserialize)]
pub struct Endpoint {
    pub rpc: String,
    pub method: String,
    pub path: String,
    pub request: Request,
    pub response: Response,
}

#[derive(Debug, Deserialize)]
pub struct Request {
    #[serde(rename = "type")]
    pub type_: String,
}

#[derive(Debug, Deserialize)]
pub struct Response {
    #[serde(rename = "type")]
    pub type_: String,
    pub cookies: Option<Vec<Cookie>>,
}

#[derive(Debug, Deserialize)]
pub struct Cookie {
    pub name: String,
    pub options: Option<CookieOptions>,
}

#[derive(Debug, Deserialize)]
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

pub enum FileFormat {
    Json,
    Yaml,
}

impl FileFormat {
    pub fn parse(&self, input: &str) -> Result<Config, Box<dyn std::error::Error>> {
        match self {
            FileFormat::Json => json::JsonParser::parse(input),
            FileFormat::Yaml => yaml::YamlParser::parse(input),
        }
    }
}
