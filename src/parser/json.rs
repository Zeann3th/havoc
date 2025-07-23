use std::fs;

use protobuf_parser::{FieldType, FileDescriptor};

use crate::parser::{Config, Field, Parser};

pub struct JsonParser;

impl Parser for JsonParser {
    fn parse(input: &str) -> Result<super::Config, Box<dyn std::error::Error>> {
        let mut config: Config = serde_json::from_str(input)?;

        for service in &mut config.spec.services {
            let proto_content = fs::read_to_string(&service.proto)?;

            let parsed = match FileDescriptor::parse(&proto_content) {
                Ok(fd) => fd,
                Err(_) => {
                    return Err(format!("Failed to parse proto file: {}", &service.proto).into());
                }
            };

            for endpoint in &mut service.endpoints {
                if let Some(request_message) = &parsed
                    .messages
                    .iter()
                    .find(|m| m.name == endpoint.request.type_)
                {
                    endpoint.request.fields = Vec::new();
                    for f in &request_message.fields {
                        let type_ = match ft_to_string(&f.typ) {
                            Ok(t) => t,
                            Err(e) => {
                                return Err(format!("Failed to convert field type: {}", e).into());
                            }
                        };
                        let field = Field {
                            name: f.name.clone(),
                            type_: type_,
                        };

                        endpoint.request.fields.push(field);
                    }
                } else {
                    return Err(format!(
                        "Request type '{}' not found in proto file",
                        endpoint.request.type_
                    )
                    .into());
                }

                if let Some(response_message) = &parsed
                    .messages
                    .iter()
                    .find(|m| m.name == endpoint.response.type_)
                {
                    endpoint.response.fields = Vec::new();
                    for f in &response_message.fields {
                        let type_ = match ft_to_string(&f.typ) {
                            Ok(t) => t,
                            Err(e) => {
                                return Err(format!("Failed to convert field type: {}", e).into());
                            }
                        };
                        let field = Field {
                            name: f.name.clone(),
                            type_: type_,
                        };

                        endpoint.response.fields.push(field);
                    }
                } else {
                    return Err(format!(
                        "Response type '{}' not found in proto file",
                        endpoint.response.type_
                    )
                    .into());
                }
            }
        }
        Ok(config)
    }
}

fn ft_to_string(ft: &FieldType) -> Result<String, Box<dyn std::error::Error>> {
    match ft {
        FieldType::Double => Ok("f64".to_string()),
        FieldType::Float => Ok("f32".to_string()),
        FieldType::Int32 => Ok("i32".to_string()),
        FieldType::Int64 => Ok("i64".to_string()),
        FieldType::Uint32 => Ok("u32".to_string()),
        FieldType::Uint64 => Ok("u64".to_string()),
        FieldType::Sint32 => Ok("i32".to_string()),
        FieldType::Sint64 => Ok("i64".to_string()),
        FieldType::Fixed32 => Ok("u32".to_string()),
        FieldType::Fixed64 => Ok("u64".to_string()),
        FieldType::Sfixed32 => Ok("i32".to_string()),
        FieldType::Sfixed64 => Ok("i64".to_string()),
        FieldType::Bool => Ok("bool".to_string()),
        FieldType::String => Ok("String".to_string()),
        FieldType::Bytes => Ok("Vec<u8>".to_string()),
        FieldType::MessageOrEnum(name) => Ok(name.clone()),
        _ => Err(format!("Unsupported field type: {:?}", ft).into()),
    }
}
