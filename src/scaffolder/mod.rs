use std::path::Path;

use crate::parser::Config;

pub mod factory;
mod filters;
mod frameworks;

pub trait Scaffolder {
    fn scaffold(config: &Config, output: &Path) -> Result<(), Box<dyn std::error::Error>>;
}
