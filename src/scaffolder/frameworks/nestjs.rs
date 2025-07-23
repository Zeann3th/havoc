#![allow(unused)]
use std::path::Path;

use crate::{parser::Config, scaffolder::Scaffolder};

pub struct NestjsScaffolder;

impl Scaffolder for NestjsScaffolder {
    fn scaffold(config: &Config, output: &Path) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}
