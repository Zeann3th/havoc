use std::{fs, path::Path};
use std::str::FromStr;

use proto_def::{lexer::Lexer, parser::Parser as ProtoParser};

use crate::{
    framework::{Framework, factory::TypeMapperFactory},
    parser::{
        Config, Field, Parser,
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

        let file_type = FileFormat::from_str(ext)?;

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

    pub fn build(&self) -> Result<Config, Box<dyn std::error::Error>> {
        let mut config = self.parse()?;
        populate_from_proto(&mut config)?;
        map_field_types(&mut config, self.framework);
        Ok(config)
    }

    fn parse(&self) -> Result<Config, Box<dyn std::error::Error>> {
        let config = match self.file_type {
            FileFormat::Json => JsonParser::parse(&self.content),
            FileFormat::Yaml => YamlParser::parse(&self.content),
        }?;
        Ok(config)
    }
}

fn populate_from_proto(config: &mut Config) -> Result<(), Box<dyn std::error::Error>> {
    for service in &mut config.spec.services {
        let proto_path = &service.proto;
        let content = fs::read_to_string(proto_path)?;
        let lexer = Lexer::new();
        let tokens = lexer
            .lex(&content)
            .map_err(|e| format!("Lexing error: {}", e))?;
        let mut parser = ProtoParser::new(&tokens);
        let proto = parser.parse()?;

        let svc = proto
            .services
            .iter()
            .find(|s| s.name == service.name)
            .ok_or_else(|| {
                format!(
                    "Service '{}' not found in proto file '{}'",
                    service.name, proto_path
                )
            })?;

        for endpoint in &mut service.endpoints {
            let rpc = svc.methods.get(&endpoint.rpc).ok_or_else(|| {
                format!(
                    "RPC method '{}' not found in service '{}'",
                    endpoint.rpc, svc.name
                )
            })?;

            if endpoint.request.type_.is_empty() {
                endpoint.request.type_ = rpc.request.clone();
            }
            if endpoint.response.type_.is_empty() {
                endpoint.response.type_ = rpc.response.clone();
            }

            let req_msg = proto
                .messages
                .iter()
                .find(|m| m.name == rpc.request)
                .ok_or_else(|| format!("Request message '{}' not found", rpc.request))?;

            endpoint.request.fields = req_msg
                .fields
                .iter()
                .map(|f| Field {
                    name: f.name.clone(),
                    type_: f.field_type.clone(),
                })
                .collect();

            let res_msg = proto
                .messages
                .iter()
                .find(|m| m.name == rpc.response)
                .ok_or_else(|| format!("Response message '{}' not found", rpc.response))?;

            endpoint.response.fields = res_msg
                .fields
                .iter()
                .map(|f| Field {
                    name: f.name.clone(),
                    type_: f.field_type.clone(),
                })
                .collect();
        }
    }

    Ok(())
}

fn map_field_types(config: &mut Config, framework: Framework) {
    let mapper = TypeMapperFactory { framework };

    for service in &mut config.spec.services {
        for endpoint in &mut service.endpoints {
            for field in &mut endpoint.request.fields {
                field.type_ = mapper.map_type(&field.type_);
            }
            for field in &mut endpoint.response.fields {
                field.type_ = mapper.map_type(&field.type_);
            }
        }
    }
}
