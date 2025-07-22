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

        #[arg(short = 'f', long, value_name = "FRAMEWORK", default_value = "axum")]
        framework: String,
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

            let template_dir = TEMPLATES
                .get_dir(&framework)
                .ok_or_else(|| format!("Framework `{}` is not supported.", framework))?;

            let output_path = Path::new(&output);
            if !output_path.exists() {
                fs::create_dir_all(output_path)?;
            }

            for file in template_dir.files() {
                let target_path = output_path.join(file.path().file_name().unwrap());
                fs::write(target_path, file.contents())?;
            }

            let gen_dir = output_path.join("src/gen");
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

            println!("✅ Project generated at `{}`", output_path.display());
            Ok(())
        }

        Command::Validate { config_path } => {
            validate(&config_path)?;
            println!("✅ Configuration is valid.");
            Ok(())
        }

        Command::ListFrameworks => {
            let frameworks = TEMPLATES
                .dirs()
                .map(|d| d.path().file_name().unwrap().to_string_lossy())
                .collect::<Vec<_>>();

            println!("Available frameworks:\n{}", frameworks.join("\n"));
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
