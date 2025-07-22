use std::collections::HashMap;
use tera::Value;

pub fn snake_case_filter(value: &Value, _: &HashMap<String, Value>) -> tera::Result<Value> {
    let name = value.as_str().ok_or("Expected a string")?;
    Ok(Value::String(to_snake_case(name)))
}

pub fn to_snake_case(name: &str) -> String {
    let mut snake = String::new();
    for (i, ch) in name.chars().enumerate() {
        if ch.is_uppercase() {
            if i > 0 {
                snake.push('_');
            }
            snake.push(ch.to_ascii_lowercase());
        } else {
            snake.push(ch);
        }
    }
    snake
}
