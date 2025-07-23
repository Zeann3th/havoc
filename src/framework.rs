use std::{fmt, str::FromStr};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
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

impl Default for Framework {
    fn default() -> Self {
        Self::Axum
    }
}

impl Framework {
    pub fn all() -> Vec<Self> {
        vec![Self::Axum, Self::NestJS]
    }
}
