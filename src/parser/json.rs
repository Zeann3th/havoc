use crate::parser::Parser;

pub struct JsonParser;

impl Parser for JsonParser {
    fn parse(input: &str) -> Result<super::Config, Box<dyn std::error::Error>> {
        let config = serde_json::from_str(input)?;
        Ok(config)
    }
}
