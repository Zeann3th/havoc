use crate::parser::formats::populate_from_proto;
use crate::parser::{Config, Parser};

pub struct JsonParser;

impl Parser for JsonParser {
    fn parse(input: &str) -> Result<Config, Box<dyn std::error::Error>> {
        let mut config: Config = serde_json::from_str(input)?;
        populate_from_proto(&mut config)?;
        Ok(config)
    }
}
