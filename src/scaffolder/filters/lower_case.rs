use std::collections::HashMap;
use tera::Value;

pub fn lower_case_filter(value: &Value, _: &HashMap<String, Value>) -> tera::Result<Value> {
    let name = value.as_str().ok_or("Expected a string")?;
    Ok(Value::String(to_lower_case(name)))
}

pub fn to_lower_case(name: &str) -> String {
    name.to_lowercase()
}
