use std::{fs, path::Path};

use clap::{Parser, Subcommand};
use include_dir::{Dir, include_dir};

use crate::parser::{Config, FileFormat};

mod parser;

static TEMPLATES: Dir = include_dir!("$CARGO_MANIFEST_DIR/templates");

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
    },

    #[command(alias = "val")]
    Validate {
        #[arg(value_name = "CONFIG_PATH")]
        config_path: String,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Command::New {
            config_path,
            output,
        } => {
            let config = validate(&config_path)?;

            for service in &config.spec.services {
                if !Path::new(&service.proto).exists() {
                    return Err(format!("Proto file {} does not exist.", service.proto).into());
                }
            }

            let output = Path::new(&output);
            if !output.exists() {
                fs::create_dir_all(&output)?;
            }

            if let Some(cargo_toml) = TEMPLATES.get_file("Cargo.toml") {
                let toml_path = output.join("Cargo.toml");
                fs::write(toml_path, cargo_toml.contents())?;
            } else {
                eprintln!("Missing Cargo.toml template in embedded templates.");
            }

            if let Some(dockerfile) = TEMPLATES.get_file("Dockerfile") {
                let dockerfile_path = output.join("Dockerfile");
                fs::write(dockerfile_path, dockerfile.contents())?;
            } else {
                eprintln!("Missing Dockerfile template in embedded templates.");
            }

            let gen_dir = output.join("src/gen");
            if !gen_dir.exists() {
                fs::create_dir_all(&gen_dir)?;
            }

            for service in &config.spec.services {
                tonic_build::configure()
                    .build_server(false)
                    .build_client(true)
                    .out_dir(&gen_dir)
                    .compile_protos(
                        &[service.proto.clone()],
                        &[Path::new(&service.proto)
                            .parent()
                            .unwrap()
                            .to_str()
                            .unwrap()],
                    )?;
            }

            Ok(())
        }
        Command::Validate { config_path } => {
            validate(&config_path)?;
            println!("Configuration is valid.");
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

    Ok(parser.parse(&contents)?)
}
