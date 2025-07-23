use clap::{Parser, Subcommand};
use std::path::Path;

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
        framework: scaffolder::Framework,
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
            let parser = parser::ParserFactory::new(&config_path)?;
            let config = parser.parse()?;

            for service in &config.spec.services {
                if !Path::new(&service.proto).exists() {
                    return Err(format!("Proto file {} does not exist.", service.proto).into());
                }
            }

            let output = Path::new(&output);
            let scaffolder_factory = scaffolder::ScaffolderFactory { framework, config };
            scaffolder_factory.scaffold(output)?;

            println!("✅ Project generated at `{}`", output.display());
            Ok(())
        }

        Command::Validate { config_path } => {
            let parser = parser::ParserFactory::new(&config_path)?;
            parser.parse()?;
            println!("✅ Configuration is valid.");
            Ok(())
        }

        Command::ListFrameworks => {
            println!("Available frameworks:");
            for fw in scaffolder::Framework::all() {
                println!("• {}", fw);
            }
            Ok(())
        }
    }
}
