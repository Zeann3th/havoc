use std::collections::HashMap;
use tera::Value;

pub fn camel_case_filter(value: &Value, _: &HashMap<String, Value>) -> tera::Result<Value> {
    let name = value.as_str().ok_or("Expected a string")?;
    Ok(Value::String(to_camel_case(name)))
}

pub fn to_camel_case(name: &str) -> String {
    let mut camel = String::new();
    let mut next_upper = false;

    for ch in name.chars() {
        if ch == '_' {
            next_upper = true;
        } else {
            if next_upper {
                camel.push(ch.to_ascii_uppercase());
                next_upper = false;
            } else {
                camel.push(ch);
            }
        }
    }

    if !camel.is_empty() {
        camel.replace_range(0..1, &camel[0..1].to_ascii_lowercase());
    }

    camel
}
