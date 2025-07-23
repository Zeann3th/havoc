use serde::{Deserialize, Serialize};

pub mod factory;
mod formats;
mod type_converter;

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
