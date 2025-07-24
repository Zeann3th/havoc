use std::path::Path;

use crate::{
    framework::Framework,
    parser::Config,
    scaffolder::{
        Scaffolder,
        frameworks::{axum::AxumScaffolder, nestjs::NestjsScaffolder, spring::SpringScaffolder},
    },
};

pub struct ScaffolderFactory {
    pub framework: Framework,
    pub config: Config,
}

impl ScaffolderFactory {
    pub fn scaffold(&self, output: &Path) -> Result<(), Box<dyn std::error::Error>> {
        match self.framework {
            Framework::Axum => AxumScaffolder::scaffold(&self.config, output),
            Framework::NestJS => NestjsScaffolder::scaffold(&self.config, output),
            Framework::Spring => SpringScaffolder::scaffold(&self.config, output),
        }
    }
}
