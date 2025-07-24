use std::fs;
use std::path::Path;

use include_dir::{Dir, include_dir};
use serde_json::json;
use tera::{Context, Tera, Value};

use crate::{
    parser::Config,
    scaffolder::{Scaffolder, filters},
};

static SPRING_TEMPLATES: Dir = include_dir!("$CARGO_MANIFEST_DIR/templates/spring");

pub struct SpringScaffolder;

impl Scaffolder for SpringScaffolder {
    fn scaffold(config: &Config, output: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let mut tera = Tera::default();
        tera.register_filter("snake_case", filters::snake_case_filter);
        tera.register_filter("camel_case", filters::camel_case_filter);
        tera.register_filter("capitalize", filters::capitalize_filter);

        render_spring_templates(&mut tera, &SPRING_TEMPLATES, output, config)?;
        Ok(())
    }
}

fn render_spring_templates(
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

            if name.contains("controller") || name.contains("client") || name.contains("service") {
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
                    context.insert("config", config);

                    let replaced_path = {
                        let file_name = rel_path.file_name().unwrap().to_string_lossy();

                        let file_stem = file_name
                            .replace(
                                "controller.java.tera",
                                &format!("{}Controller.java", service.name),
                            )
                            .replace(
                                "service.java.tera",
                                &format!("{}Service.java", service.name),
                            )
                            .replace("client.java.tera", &format!("{}Client.java", service.name));

                        if name.contains("controller") {
                            format!("controller/{}", file_stem)
                        } else if name.contains("service") || name.contains("client") {
                            format!("service/{}", file_stem)
                        } else {
                            file_stem
                        }
                    };

                    let output_path = output
                        .join("src/main/java/com/example/gateway")
                        .join(Path::new(&replaced_path));

                    if let Some(parent) = output_path.parent() {
                        fs::create_dir_all(parent)?;
                    }

                    let rendered = tera.render(&name, &context)?;
                    fs::write(output_path, rendered)?;
                }

                continue;
            }

            let mut context = Context::new();
            context.insert("config", config);
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
        render_spring_templates(tera, subdir, output, config)?;
    }

    Ok(())
}