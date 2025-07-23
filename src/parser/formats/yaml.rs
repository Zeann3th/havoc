use crate::parser::{Config, Parser, formats::populate_from_proto};

pub struct YamlParser;

impl Parser for YamlParser {
    fn parse(input: &str) -> Result<Config, Box<dyn std::error::Error>> {
        let mut config: Config = serde_yml::from_str(input)?;
        populate_from_proto(&mut config)?;
        Ok(config)
    }
}
