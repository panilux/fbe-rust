//! Fast Binary Encoding compiler for Rust
//! 
//! Generates Rust code from .fbe schema files
//! HERSEY DAHA IYI BIR PANILUX ICIN! 🚀

use regex::Regex;
use std::fs;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() < 3 {
        eprintln!("Usage: fbec <input.fbe> <output_dir>");
        std::process::exit(1);
    }

    let input_file = &args[1];
    let output_dir = &args[2];

    match generate(input_file, output_dir) {
        Ok(_) => println!("✓ Code generation successful!"),
        Err(e) => {
            eprintln!("✗ Error: {}", e);
            std::process::exit(1);
        }
    }
}

fn generate(input_file: &str, output_dir: &str) -> Result<(), String> {
    let content = fs::read_to_string(input_file)
        .map_err(|e| format!("Failed to read {}: {}", input_file, e))?;

    fs::create_dir_all(output_dir)
        .map_err(|e| format!("Failed to create output directory: {}", e))?;

    let mut generator = Generator::new();
    generator.parse(&content)?;
    generator.generate_code(output_dir)?;

    println!("Generated {} enums, {} flags, {} structs", 
        generator.enums.len(), generator.flags.len(), generator.structs.len());

    Ok(())
}

struct Generator {
    enums: Vec<EnumDef>,
    flags: Vec<FlagsDef>,
    structs: Vec<StructDef>,
}

#[derive(Debug, Clone)]
struct EnumDef {
    name: String,
    base_type: String,
    values: Vec<(String, String)>,
}

#[derive(Debug, Clone)]
struct FlagsDef {
    name: String,
    base_type: String,
    values: Vec<(String, String)>,
}

#[derive(Debug, Clone)]
struct StructDef {
    name: String,
    fields: Vec<FieldDef>,
}

#[derive(Debug, Clone)]
struct FieldDef {
    name: String,
    fbe_type: String,
    is_optional: bool,
    is_array: bool,
}

impl Generator {
    fn new() -> Self {
        Self {
            enums: Vec::new(),
            flags: Vec::new(),
            structs: Vec::new(),
        }
    }

    fn parse(&mut self, content: &str) -> Result<(), String> {
        self.parse_enums(content)?;
        self.parse_flags(content)?;
        self.parse_structs(content)?;
        Ok(())
    }

    fn parse_enums(&mut self, content: &str) -> Result<(), String> {
        let re = Regex::new(r"enum\s+(\w+)\s*:\s*(\w+)\s*\{([^}]+)\}").unwrap();
        
        for cap in re.captures_iter(content) {
            let name = cap[1].to_string();
            let base_type = cap[2].to_string();
            let body = &cap[3];

            let mut values = Vec::new();
            let mut index = 0;
            for line in body.lines() {
                let line = line.trim();
                if line.is_empty() || line.starts_with("//") {
                    continue;
                }

                if let Some((key, val)) = line.split_once('=') {
                    let key = key.trim().to_string();
                    let val = val.trim().trim_end_matches(';').trim().to_string();
                    values.push((key, val));
                    index += 1;
                } else {
                    let key = line.trim_end_matches(';').trim().to_string();
                    if !key.is_empty() {
                        values.push((key, index.to_string()));
                        index += 1;
                    }
                }
            }

            self.enums.push(EnumDef { name, base_type, values });
        }

        Ok(())
    }

    fn parse_flags(&mut self, content: &str) -> Result<(), String> {
        let re = Regex::new(r"flags\s+(\w+)\s*:\s*(\w+)\s*\{([^}]+)\}").unwrap();
        
        for cap in re.captures_iter(content) {
            let name = cap[1].to_string();
            let base_type = cap[2].to_string();
            let body = &cap[3];

            let mut values = Vec::new();
            for line in body.lines() {
                let line = line.trim();
                if line.is_empty() || line.starts_with("//") {
                    continue;
                }

                if let Some((key, val)) = line.split_once('=') {
                    let key = key.trim().to_string();
                    let val = val.trim().trim_end_matches(';').trim().to_string();
                    values.push((key, val));
                }
            }

            self.flags.push(FlagsDef { name, base_type, values });
        }

        Ok(())
    }

    fn parse_structs(&mut self, content: &str) -> Result<(), String> {
        let re = Regex::new(r"struct\s+(\w+)(?:\(\d+\))?\s*\{([^}]+)\}").unwrap();
        
        for cap in re.captures_iter(content) {
            let name = cap[1].to_string();
            let body = &cap[2];

            let mut fields = Vec::new();
            let field_re = Regex::new(r"(\w+)(\?)?(\[\])?\s+(\w+)").unwrap();

            for line in body.lines() {
                let line = line.trim();
                if line.is_empty() || line.starts_with("//") || line.starts_with("[") {
                    continue;
                }

                if let Some(cap) = field_re.captures(line) {
                    let fbe_type = cap[1].to_string();
                    let is_optional = cap.get(2).is_some();
                    let is_array = cap.get(3).is_some();
                    let name = cap[4].to_string();

                    fields.push(FieldDef {
                        name,
                        fbe_type,
                        is_optional,
                        is_array,
                    });
                }
            }

            self.structs.push(StructDef { name, fields });
        }

        Ok(())
    }

    fn generate_code(&self, output_dir: &str) -> Result<(), String> {
        for enum_def in &self.enums {
            self.generate_enum(enum_def, output_dir)?;
        }

        for flags_def in &self.flags {
            self.generate_flags(flags_def, output_dir)?;
        }

        for struct_def in &self.structs {
            self.generate_struct(struct_def, output_dir)?;
        }

        self.generate_mod_file(output_dir)?;
        Ok(())
    }

    fn generate_enum(&self, enum_def: &EnumDef, output_dir: &str) -> Result<(), String> {
        let file_name = format!("{}/{}.rs", output_dir, to_snake_case(&enum_def.name));
        let rust_type = map_fbe_type(&enum_def.base_type);

        let mut code = format!("//! {} enum\n\n", enum_def.name);
        code.push_str("#[derive(Debug, Clone, Copy, PartialEq, Eq)]\n");
        code.push_str(&format!("#[repr({})]\n", rust_type));
        code.push_str(&format!("pub enum {} {{\n", enum_def.name));
        
        for (key, _) in &enum_def.values {
            code.push_str(&format!("    {},\n", to_pascal_case(&escape_keyword(key))));
        }
        code.push_str("}\n");

        fs::write(&file_name, code)
            .map_err(|e| format!("Failed to write {}: {}", file_name, e))?;
        Ok(())
    }

    fn generate_flags(&self, flags_def: &FlagsDef, output_dir: &str) -> Result<(), String> {
        let file_name = format!("{}/{}.rs", output_dir, to_snake_case(&flags_def.name));
        let rust_type = map_fbe_type(&flags_def.base_type);

        let mut code = format!("//! {} flags\n\n", flags_def.name);
        for (key, val) in &flags_def.values {
            let const_name = to_screaming_snake_case(&escape_keyword(key));
            code.push_str(&format!("pub const {}: {} = {};\n", const_name, rust_type, val));
        }

        fs::write(&file_name, code)
            .map_err(|e| format!("Failed to write {}: {}", file_name, e))?;
        Ok(())
    }

    fn generate_struct(&self, struct_def: &StructDef, output_dir: &str) -> Result<(), String> {
        let file_name = format!("{}/{}.rs", output_dir, to_snake_case(&struct_def.name));

        let mut code = format!("//! {} struct\n\n", struct_def.name);
        code.push_str("use crate::buffer::{{WriteBuffer, ReadBuffer}};\n\n");
        code.push_str("#[derive(Debug, Clone, Default)]\n");
        code.push_str(&format!("pub struct {} {{\n", struct_def.name));
        
        for field in &struct_def.fields {
            let rust_type = self.map_field_type(field);
            code.push_str(&format!("    pub {}: {},\n", field.name, rust_type));
        }
        code.push_str("}\n\n");

        // Add serialization methods
        code.push_str(&format!("impl {} {{\n", struct_def.name));
        
        // Serialize method
        code.push_str("    pub fn serialize(&self, buffer: &mut WriteBuffer) -> usize {\n");
        code.push_str("        let mut offset = 0;\n");
        for field in &struct_def.fields {
            code.push_str(&self.generate_serialize_field(field));
        }
        code.push_str("        offset\n");
        code.push_str("    }\n\n");

        // Deserialize method
        code.push_str("    pub fn deserialize(buffer: &ReadBuffer) -> Self {\n");
        code.push_str("        let mut offset = 0;\n");
        code.push_str("        Self {\n");
        for field in &struct_def.fields {
            code.push_str(&self.generate_deserialize_field(field));
        }
        code.push_str("        }\n");
        code.push_str("    }\n");
        code.push_str("}\n");

        fs::write(&file_name, code)
            .map_err(|e| format!("Failed to write {}: {}", file_name, e))?;
        Ok(())
    }

    fn generate_serialize_field(&self, field: &FieldDef) -> String {
        let write_method = get_write_method(&field.fbe_type);
        let size = get_type_size(&field.fbe_type);
        
        if field.fbe_type == "string" {
            format!("        buffer.write_string(offset, &self.{});\n        offset += 4 + self.{}.len();\n", field.name, field.name)
        } else {
            format!("        buffer.{}(offset, self.{});\n        offset += {};\n", write_method, field.name, size)
        }
    }

    fn generate_deserialize_field(&self, field: &FieldDef) -> String {
        let read_method = get_read_method(&field.fbe_type);
        
        if field.fbe_type == "string" {
            format!("            {}: {{ let val = buffer.{}(offset); offset += 4 + val.len(); val }},\n", field.name, read_method)
        } else {
            let size = get_type_size(&field.fbe_type);
            format!("            {}: {{ let val = buffer.{}(offset); offset += {}; val }},\n", field.name, read_method, size)
        }
    }

    fn map_field_type(&self, field: &FieldDef) -> String {
        let base_type = map_fbe_type(&field.fbe_type);
        
        if field.is_array {
            if field.is_optional {
                format!("Option<Vec<{}>>", base_type)
            } else {
                format!("Vec<{}>", base_type)
            }
        } else if field.is_optional {
            format!("Option<{}>", base_type)
        } else {
            base_type
        }
    }

    fn generate_mod_file(&self, output_dir: &str) -> Result<(), String> {
        let mut code = String::from("//! Generated FBE modules\n\n");

        for enum_def in &self.enums {
            code.push_str(&format!("pub mod {};\n", to_snake_case(&enum_def.name)));
        }
        for flags_def in &self.flags {
            code.push_str(&format!("pub mod {};\n", to_snake_case(&flags_def.name)));
        }
        for struct_def in &self.structs {
            code.push_str(&format!("pub mod {};\n", to_snake_case(&struct_def.name)));
        }

        fs::write(format!("{}/mod.rs", output_dir), code)
            .map_err(|e| format!("Failed to write mod.rs: {}", e))?;
        Ok(())
    }
}

fn map_fbe_type(fbe_type: &str) -> String {
    match fbe_type {
        "bool" => "bool",
        "byte" | "int8" => "i8",
        "uint8" => "u8",
        "int16" => "i16",
        "uint16" => "u16",
        "int32" => "i32",
        "uint32" => "u32",
        "int64" => "i64",
        "uint64" => "u64",
        "float" => "f32",
        "double" => "f64",
        "string" => "String",
        _ => fbe_type,
    }.to_string()
}

fn to_snake_case(s: &str) -> String {
    let mut result = String::new();
    for (i, ch) in s.chars().enumerate() {
        if ch.is_uppercase() && i > 0 {
            result.push('_');
        }
        result.push(ch.to_lowercase().next().unwrap());
    }
    result
}

fn to_pascal_case(s: &str) -> String {
    s.chars().next().unwrap().to_uppercase().to_string() + &s[1..]
}

fn to_screaming_snake_case(s: &str) -> String {
    to_snake_case(s).to_uppercase()
}

fn escape_keyword(s: &str) -> String {
    match s {
        "type" | "match" | "loop" | "move" | "ref" | "self" | "Self" => format!("r#{}", s),
        _ => s.to_string()
    }
}


fn get_write_method(fbe_type: &str) -> String {
    match fbe_type {
        "bool" => "write_bool",
        "byte" | "int8" => "write_i8",
        "uint8" => "write_u8",
        "int16" => "write_i16",
        "uint16" => "write_u16",
        "int32" => "write_i32",
        "uint32" => "write_u32",
        "int64" => "write_i64",
        "uint64" => "write_u64",
        "float" => "write_f32",
        "double" => "write_f64",
        "string" => "write_string",
        _ => "write_i32",
    }.to_string()
}

fn get_read_method(fbe_type: &str) -> String {
    match fbe_type {
        "bool" => "read_bool",
        "byte" | "int8" => "read_i8",
        "uint8" => "read_u8",
        "int16" => "read_i16",
        "uint16" => "read_u16",
        "int32" => "read_i32",
        "uint32" => "read_u32",
        "int64" => "read_i64",
        "uint64" => "read_u64",
        "float" => "read_f32",
        "double" => "read_f64",
        "string" => "read_string",
        _ => "read_i32",
    }.to_string()
}

fn get_type_size(fbe_type: &str) -> usize {
    match fbe_type {
        "bool" | "byte" | "int8" | "uint8" => 1,
        "int16" | "uint16" => 2,
        "int32" | "uint32" | "float" => 4,
        "int64" | "uint64" | "double" => 8,
        _ => 4,
    }
}
