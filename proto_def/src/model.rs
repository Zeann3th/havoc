use std::collections::HashMap;

#[derive(Debug)]
pub struct Proto {
    pub package: Option<String>,
    pub syntax: Option<String>,
    pub options: HashMap<String, String>,
    pub imports: Vec<String>,
    pub services: Vec<Service>,
    pub messages: Vec<Message>,
}

impl Default for Proto {
    fn default() -> Self {
        Proto {
            package: None,
            syntax: Some("proto3".to_string()),
            options: HashMap::new(),
            imports: Vec::new(),
            services: Vec::new(),
            messages: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub struct Service {
    pub name: String,
    pub methods: HashMap<String, RpcMethod>,
}

#[derive(Debug, PartialEq)]
pub struct RpcMethod {
    pub name: String,
    pub request: String,
    pub response: String,
}

#[derive(Debug)]
pub struct Message {
    pub name: String,
    pub fields: Vec<Field>,
}

#[derive(Debug, PartialEq)]
pub struct Field {
    pub name: String,
    pub field_type: String,
    pub number: u32,
    pub repeated: bool,
}
