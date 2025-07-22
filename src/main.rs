use std::{fs, path::Path};

use crate::parser::{Config, FileFormat};
use crate::scaffolder::Framework;
use clap::{Parser, Subcommand};

mod parser;
mod scaffolder;

#[derive(Parser)]
#[command(name = "havoc", version, about = "A gRPC Gateway generator CLI tool")]
struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
enum Command {
    #[command(alias = "new")]
    New {
        #[arg(value_name = "CONFIG_PATH")]
        config_path: String,

        #[arg(short, long, value_name = "OUTPUT_DIR", default_value = "gateway")]
        output: String,

        #[arg(short = 'f', long, value_name = "FRAMEWORK", default_value = "axum")]
        framework: Framework,
    },

    #[command(alias = "val")]
    Validate {
        #[arg(value_name = "CONFIG_PATH")]
        config_path: String,
    },

    #[command(alias = "list-fw")]
    ListFrameworks,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Command::New {
            config_path,
            output,
            framework,
        } => {
            let config = validate(&config_path)?;

            for service in &config.spec.services {
                if !Path::new(&service.proto).exists() {
                    return Err(format!("Proto file {} does not exist.", service.proto).into());
                }
            }

            let output_path = Path::new(&output);
            framework.scaffold(&config, output_path)?;

            println!("✅ Project generated at `{}`", output_path.display());
            Ok(())
        }

        Command::Validate { config_path } => {
            validate(&config_path)?;
            println!("✅ Configuration is valid.");
            Ok(())
        }

        Command::ListFrameworks => {
            println!("Available frameworks:");
            for fw in Framework::all() {
                println!("• {}", fw);
            }
            Ok(())
        }
    }
}

fn validate(config_path: &str) -> Result<Config, Box<dyn std::error::Error>> {
    let contents = fs::read_to_string(config_path)?;
    let ext = Path::new(config_path)
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or_default();

    let parser = match ext {
        "json" => FileFormat::Json,
        "yaml" | "yml" => FileFormat::Yaml,
        _ => return Err(format!("Unsupported file format: {}", ext).into()),
    };

    parser.parse(&contents)
}
