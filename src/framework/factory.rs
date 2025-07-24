use crate::framework::{
    type_mapper::{JavaTypeMapper, RustTypeMapper, TypeMapper}, Framework
};

pub struct TypeMapperFactory {
    pub framework: Framework,
}

impl TypeMapperFactory {
    pub fn map_type(&self, proto_type: &str) -> String {
        match self.framework {
            Framework::Axum => RustTypeMapper.map_type(proto_type),
            Framework::NestJS => "NOT IMPLEMENTED".to_string(),
            Framework::Spring => JavaTypeMapper.map_type(proto_type),
        }
    }
}
