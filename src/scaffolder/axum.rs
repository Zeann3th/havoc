use std::{
    fs,
    path::{Path, PathBuf},
};

use include_dir::{Dir, include_dir};
use serde_json::json;
use tera::{Context, Tera, Value};

use crate::{
    parser::Config,
    scaffolder::{Scaffolder, filters},
};

static AXUM_TEMPLATES: Dir = include_dir!("$CARGO_MANIFEST_DIR/templates/axum");

pub struct AxumScaffolder;

impl Scaffolder for AxumScaffolder {
    fn scaffold(config: &Config, output: &Path) -> Result<(), Box<dyn std::error::Error>> {
        if !output.exists() {
            fs::create_dir_all(output)?;
        }

        for file in AXUM_TEMPLATES.files() {
            let target_path = output.join(file.path().file_name().unwrap());
            fs::write(target_path, file.contents())?;
        }

        let gen_dir = output.join("src/gen");
        if !gen_dir.exists() {
            fs::create_dir_all(&gen_dir)?;
        }

        for service in &config.spec.services {
            let proto_path = PathBuf::from(&service.proto);
            if !proto_path.exists() {
                return Err(format!("Proto file {} does not exist.", service.proto).into());
            }

            let proto_dir = proto_path
                .parent()
                .ok_or("Could not resolve parent directory of proto")?;

            tonic_build::configure()
                .build_server(false)
                .build_client(true)
                .out_dir(&gen_dir)
                .compile_protos(&[proto_path.clone()], &[proto_dir])?;
        }

        let mut tera = Tera::default();
        tera.register_filter("snake_case", filters::snake_case_filter);
        render_templates_recursively(&mut tera, &AXUM_TEMPLATES, output, config)?;

        Ok(())
    }
}

fn render_templates_recursively(
    tera: &mut Tera,
    dir: &Dir,
    output: &Path,
    config: &Config,
) -> Result<(), Box<dyn std::error::Error>> {
    for file in dir.files() {
        let rel_path = file.path();
        let is_tera = rel_path.extension().and_then(|s| s.to_str()) == Some("tera");

        if is_tera {
            let name = rel_path.to_string_lossy();
            let content = std::str::from_utf8(file.contents())?;
            tera.add_raw_template(&name, content)?;

            if name.contains("http/service") && name.ends_with("mod.rs.tera") {
                for service in &config.spec.services {
                    let mut patched_service = serde_json::to_value(service)?.clone();

                    if let Some(endpoints) = patched_service
                        .get_mut("endpoints")
                        .and_then(Value::as_array_mut)
                    {
                        for endpoint in endpoints {
                            endpoint
                                .get_mut("request")
                                .and_then(Value::as_object_mut)
                                .map(|req| req.entry("fields").or_insert(json!([])));

                            endpoint
                                .get_mut("response")
                                .and_then(Value::as_object_mut)
                                .map(|res| res.entry("fields").or_insert(json!([])));
                        }
                    }

                    let mut context = Context::new();
                    context.insert("service", &patched_service);

                    let snake_name = filters::to_snake_case(&service.name);
                    let replaced_path = rel_path.to_string_lossy().replace("service", &snake_name);
                    let output_file = output.join(replaced_path).with_extension("");

                    if let Some(parent) = output_file.parent() {
                        fs::create_dir_all(parent)?;
                    }

                    let rendered = tera.render(&name, &context)?;
                    fs::write(output_file, rendered)?;
                }

                continue;
            }

            let mut context = Context::new();
            context.insert("services", &config.spec.services);

            let target_path = output.join(rel_path).with_extension("");
            if let Some(parent) = target_path.parent() {
                fs::create_dir_all(parent)?;
            }

            let rendered = tera.render(&name, &context)?;
            fs::write(target_path, rendered)?;
        } else {
            let target_path = output.join(rel_path);
            if let Some(parent) = target_path.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::write(target_path, file.contents())?;
        }
    }

    for subdir in dir.dirs() {
        render_templates_recursively(tera, subdir, output, config)?;
    }

    Ok(())
}
