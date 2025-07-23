use crate::parser::{Config, Parser};

pub struct YamlParser;

impl Parser for YamlParser {
    fn parse(input: &str) -> Result<Config, Box<dyn std::error::Error>> {
        let config: Config = serde_yml::from_str(input)?;
        Ok(config)
    }
}
