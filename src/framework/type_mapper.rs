pub trait TypeMapper {
    fn map_type(&self, proto_type: &str) -> String;
}

pub struct RustTypeMapper;

impl TypeMapper for RustTypeMapper {
    fn map_type(&self, proto_type: &str) -> String {
        match proto_type {
            "string" => "String".to_string(),
            "int32" | "sint32" | "fixed32" => "i32".to_string(),
            "int64" | "sint64" | "fixed64" => "i64".to_string(),
            "bool" => "bool".to_string(),
            "float" => "f32".to_string(),
            "double" => "f64".to_string(),
            "bytes" => "Vec<u8>".to_string(),
            "repeated" => "Vec<_>".to_string(),
            "uint32" => "u32".to_string(),
            "uint64" => "u64".to_string(),
            _ => proto_type.to_string(),
        }
    }
}
