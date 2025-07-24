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

pub struct JavaTypeMapper;

impl TypeMapper for JavaTypeMapper {
    fn map_type(&self, proto_type: &str) -> String {
        match proto_type {
            "string" => "String".to_string(),
            "int32" | "sint32" | "fixed32" | "uint32" => "int".to_string(),
            "int64" | "sint64" | "fixed64" | "uint64" => "long".to_string(),
            "bool" => "boolean".to_string(),
            "float" => "float".to_string(),
            "double" => "double".to_string(),
            "bytes" => "byte[]".to_string(),
            "repeated" => "List<...>".to_string(),
            _ => proto_type.to_string(),
        }
    }
}

pub struct TypeScriptTypeMapper;

impl TypeMapper for TypeScriptTypeMapper {
    fn map_type(&self, proto_type: &str) -> String {
        match proto_type {
            "string" => "string".to_string(),
            "int32" | "sint32" | "fixed32" | "uint32" => "number".to_string(),
            "int64" | "sint64" | "fixed64" | "uint64" => "bigint".to_string(),
            "bool" => "boolean".to_string(),
            "float" => "number".to_string(),
            "double" => "number".to_string(),
            "bytes" => "Uint8Array".to_string(),
            "repeated" => "Array<...>".to_string(),
            _ => proto_type.to_string(),
        }
    }
}
