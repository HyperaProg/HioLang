use std::collections::HashMap;
use std::path::Path;
use std::fs;

#[derive(Debug, Clone)]
pub struct LibraryFunction {
    pub name: String,
    pub params: Vec<String>,
    pub return_type: String,
    pub implementation_language: String, // C, C++, Rust, Go, etc.
    pub source_code: String,
}

#[derive(Debug, Clone)]
pub struct HioCLibrary {
    pub name: String,
    pub version: String,
    pub description: String,
    pub functions: HashMap<String, LibraryFunction>,
    pub language: String,
}

impl HioCLibrary {
    pub fn new(name: String, version: String, description: String, language: String) -> Self {
        HioCLibrary {
            name,
            version,
            description,
            functions: HashMap::new(),
            language,
        }
    }
    
    pub fn add_function(&mut self, function: LibraryFunction) {
        self.functions.insert(function.name.clone(), function);
    }
    
    pub fn get_function(&self, name: &str) -> Option<&LibraryFunction> {
        self.functions.get(name)
    }
    
    pub fn export_to_json(&self) -> String {
        let mut json = format!(
            r#"{{
  "name": "{}",
  "version": "{}",
  "description": "{}",
  "language": "{}",
  "functions": {{
"#,
            self.name, self.version, self.description, self.language
        );
        
        let mut first = true;
        for (_, func) in &self.functions {
            if !first {
                json.push_str(",\n");
            }
            json.push_str(&format!(
                r#"    "{}": {{
      "params": [{}],
      "return_type": "{}",
      "implementation_language": "{}"
    }}"#,
                func.name,
                func.params.iter()
                    .map(|p| format!(r#""{}""#, p))
                    .collect::<Vec<_>>()
                    .join(", "),
                func.return_type,
                func.implementation_language
            ));
            first = false;
        }
        
        json.push_str("\n  }\n}");
        json
    }
}

pub struct LibraryManager {
    libraries: HashMap<String, HioCLibrary>,
}

impl LibraryManager {
    pub fn new() -> Self {
        LibraryManager {
            libraries: HashMap::new(),
        }
    }
    
    pub fn register_library(&mut self, lib: HioCLibrary) {
        self.libraries.insert(lib.name.clone(), lib);
    }
    
    pub fn get_library(&self, name: &str) -> Option<&HioCLibrary> {
        self.libraries.get(name)
    }
    
    pub fn list_libraries(&self) -> Vec<String> {
        self.libraries.keys().cloned().collect()
    }
    
    pub fn load_from_file(&mut self, path: &Path) -> Result<(), String> {
        if let Ok(content) = fs::read_to_string(path) {
            self.parse_library_definition(&content)?;
            Ok(())
        } else {
            Err(format!("Failed to read library file: {:?}", path))
        }
    }
    
    pub fn save_library(&self, name: &str, path: &Path) -> Result<(), String> {
        if let Some(lib) = self.get_library(name) {
            let json = lib.export_to_json();
            fs::write(path, json)
                .map_err(|e| format!("Failed to write library file: {}", e))
        } else {
            Err(format!("Library not found: {}", name))
        }
    }
    
    fn parse_library_definition(&mut self, content: &str) -> Result<(), String> {
        // Simple parser for library definitions
        // In a real implementation, this would parse JSON or a custom format
        
        if let Some(name_idx) = content.find("name=") {
            let rest = &content[name_idx + 5..];
            if let Some(end_idx) = rest.find(';') {
                let lib_name = rest[..end_idx].trim().to_string();
                let lib = HioCLibrary::new(
                    lib_name,
                    "1.0.0".to_string(),
                    "Auto-loaded library".to_string(),
                    "mixed".to_string(),
                );
                self.register_library(lib);
            }
        }
        
        Ok(())
    }
}

// Built-in standard library functions
pub fn create_stdlib_c() -> HioCLibrary {
    let mut lib = HioCLibrary::new(
        "stdlib_c".to_string(),
        "1.0.0".to_string(),
        "Standard library implemented in C".to_string(),
        "C".to_string(),
    );
    
    lib.add_function(LibraryFunction {
        name: "strlen".to_string(),
        params: vec!["str".to_string()],
        return_type: "number".to_string(),
        implementation_language: "C".to_string(),
        source_code: r#"
#include <string.h>
int strlen_hio(const char* str) {
    return strlen(str);
}
"#.to_string(),
    });
    
    lib.add_function(LibraryFunction {
        name: "strcpy".to_string(),
        params: vec!["dest".to_string(), "src".to_string()],
        return_type: "string".to_string(),
        implementation_language: "C".to_string(),
        source_code: r#"
#include <string.h>
char* strcpy_hio(char* dest, const char* src) {
    return strcpy(dest, src);
}
"#.to_string(),
    });
    
    lib
}

pub fn create_stdlib_cpp() -> HioCLibrary {
    let mut lib = HioCLibrary::new(
        "stdlib_cpp".to_string(),
        "1.0.0".to_string(),
        "Standard library implemented in C++".to_string(),
        "C++".to_string(),
    );
    
    lib.add_function(LibraryFunction {
        name: "string_length".to_string(),
        params: vec!["str".to_string()],
        return_type: "number".to_string(),
        implementation_language: "C++".to_string(),
        source_code: r#"
#include <string>
int string_length(const std::string& str) {
    return str.length();
}
"#.to_string(),
    });
    
    lib
}

pub fn create_stdlib_rust() -> HioCLibrary {
    let mut lib = HioCLibrary::new(
        "stdlib_rust".to_string(),
        "1.0.0".to_string(),
        "Standard library implemented in Rust".to_string(),
        "Rust".to_string(),
    );
    
    lib.add_function(LibraryFunction {
        name: "string_reverse".to_string(),
        params: vec!["str".to_string()],
        return_type: "string".to_string(),
        implementation_language: "Rust".to_string(),
        source_code: r#"
pub fn string_reverse(s: &str) -> String {
    s.chars().rev().collect()
}
"#.to_string(),
    });
    
    lib
}

pub fn create_stdlib_go() -> HioCLibrary {
    let mut lib = HioCLibrary::new(
        "stdlib_go".to_string(),
        "1.0.0".to_string(),
        "Standard library implemented in Go".to_string(),
        "Go".to_string(),
    );
    
    lib.add_function(LibraryFunction {
        name: "bytes_to_string".to_string(),
        params: vec!["data".to_string()],
        return_type: "string".to_string(),
        implementation_language: "Go".to_string(),
        source_code: r#"
func BytesToString(data []byte) string {
    return string(data)
}
"#.to_string(),
    });
    
    lib
}
