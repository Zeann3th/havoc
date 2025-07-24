use std::collections::HashMap;
use tera::Value;

pub fn capitalize_filter(value: &Value, _: &HashMap<String, Value>) -> tera::Result<Value> {
    let name = value.as_str().ok_or("Expected a string")?;
    Ok(Value::String(to_capitalize(name)))
}

pub fn to_capitalize(name: &str) -> String {
    if name.is_empty() {
        return String::new();
    }
    let mut chars = name.chars();
    let first_char = chars.next().unwrap().to_uppercase().to_string();
    first_char + chars.as_str()
}
