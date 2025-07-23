use std::fs;

use proto_def::lexer::Lexer;

use crate::parser::{Config, Field};
use proto_def::parser::Parser as ProtoParser;

pub mod json;
pub mod yaml;

pub enum FileFormat {
    Json,
    Yaml,
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
