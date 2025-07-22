use crate::parser::Parser;

pub struct YamlParser;

impl Parser for YamlParser {
    fn parse(input: &str) -> Result<super::Config, Box<dyn std::error::Error>> {
        let config = serde_yml::from_str(input)?;
        Ok(config)
    }
}
