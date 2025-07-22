use std::path::Path;

use crate::parser::Config;

pub mod axum;
mod filters;
pub mod nestjs;

pub trait Scaffolder {
    fn scaffold(config: &Config, output: &Path) -> Result<(), Box<dyn std::error::Error>>;
}

use std::{fmt, str::FromStr};

#[derive(Debug, Clone, Copy)]
pub enum Framework {
    Axum,
    NestJS,
}

impl FromStr for Framework {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "axum" => Ok(Self::Axum),
            "nestjs" => Ok(Self::NestJS),
            _ => Err(format!("Unsupported framework: {}", s)),
        }
    }
}

impl fmt::Display for Framework {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Self::Axum => "axum",
            Self::NestJS => "nestjs",
        };
        write!(f, "{}", name)
    }
}

impl Framework {
    pub fn all() -> Vec<Self> {
        vec![Self::Axum, Self::NestJS]
    }

    pub fn scaffold(
        &self,
        config: &Config,
        output: &Path,
    ) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            Framework::Axum => axum::AxumScaffolder::scaffold(config, output),
            Framework::NestJS => nestjs::NestjsScaffolder::scaffold(config, output),
        }
    }
}
