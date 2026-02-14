mod lexer;
mod parser;
mod ast;
mod interpreter;
mod compiler;
mod library;

use std::env;
use std::fs;
use std::path::Path;

use parser::Parser;
use interpreter::Interpreter;
use compiler::Compiler;
use library::{LibraryManager, create_stdlib_c, create_stdlib_cpp, create_stdlib_rust, create_stdlib_go};

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        print_help();
        return;
    }
    
    let command = &args[1];
    
    match command.as_str() {
        "run" => {
            if args.len() < 3 {
                eprintln!("Usage: hiolang run <file>");
                return;
            }
            run_file(&args[2]);
        }
        "compile" => {
            if args.len() < 3 {
                eprintln!("Usage: hiolang compile <file> [output]");
                return;
            }
            compile_file(&args[2], args.get(3).map(|s| s.as_str()));
        }
        "lib" => {
            if args.len() < 3 {
                list_libraries();
            } else {
                match args[2].as_str() {
                    "info" => {
                        if args.len() >= 4 {
                            show_library_info(&args[3]);
                        }
                    }
                    "create" => {
                        if args.len() >= 5 {
                            create_library(&args[3], &args[4]);
                        }
                    }
                    _ => eprintln!("Unknown library command"),
                }
            }
        }
        "repl" => {
            start_repl();
        }
        "version" => {
            println!("Hiolang v0.1.0");
        }
        "help" | "-h" | "--help" => {
            print_help();
        }
        _ => {
            eprintln!("Unknown command: {}", command);
            print_help();
        }
    }
}

fn print_help() {
    println!("Hiolang Compiler/Interpreter v0.1.0");
    println!();
    println!("USAGE:");
    println!("    hiolang <COMMAND> [ARGS]");
    println!();
    println!("COMMANDS:");
    println!("    run <FILE>              Run a Hiolang file");
    println!("    compile <FILE> [OUT]    Compile to bytecode");
    println!("    lib                     List available libraries");
    println!("    lib info <NAME>         Show library information");
    println!("    lib create <NAME> <LANG> Create a new library");
    println!("    repl                    Start interactive REPL");
    println!("    version                 Show version");
    println!("    help                    Show this help message");
}

fn run_file(filename: &str) {
    let path = Path::new(filename);
    
    match fs::read_to_string(path) {
        Ok(content) => {
            match execute_code(&content) {
                Ok(result) => {
                    println!("Result: {}", result.to_string());
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to read file {}: {}", filename, e);
        }
    }
}

fn execute_code(code: &str) -> Result<ast::Value, String> {
    let mut parser = Parser::new(code);
    let program = parser.parse()?;
    
    let mut interpreter = Interpreter::new();
    interpreter.interpret(&program)
}

fn compile_file(filename: &str, output: Option<&str>) {
    let path = Path::new(filename);
    
    match fs::read_to_string(path) {
        Ok(content) => {
            let mut parser = Parser::new(&content);
            match parser.parse() {
                Ok(program) => {
                    let mut compiler = Compiler::new();
                    match compiler.compile(&program) {
                        Ok(bytecode) => {
                            let output_file = output.unwrap_or("a.hio");
                            match serialize_bytecode(&bytecode, output_file) {
                                Ok(_) => {
                                    println!("Successfully compiled to {}", output_file);
                                }
                                Err(e) => {
                                    eprintln!("Failed to write bytecode: {}", e);
                                }
                            }
                        }
                        Err(e) => {
                            eprintln!("Compilation error: {}", e);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Parse error: {}", e);
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to read file {}: {}", filename, e);
        }
    }
}

fn serialize_bytecode(bytecode: &[compiler::BytecodeOp], filename: &str) -> Result<(), String> {
    let serialized = format!("{:#?}", bytecode);
    fs::write(filename, serialized)
        .map_err(|e| format!("IO error: {}", e))?;
    Ok(())
}

fn list_libraries() {
    let mut manager = LibraryManager::new();
    
    // Register built-in libraries
    manager.register_library(create_stdlib_c());
    manager.register_library(create_stdlib_cpp());
    manager.register_library(create_stdlib_rust());
    manager.register_library(create_stdlib_go());
    
    println!("Available Libraries:");
    for lib_name in manager.list_libraries() {
        if let Some(lib) = manager.get_library(&lib_name) {
            println!("  {} v{} ({})", lib.name, lib.version, lib.language);
            println!("    {}", lib.description);
        }
    }
}

fn show_library_info(lib_name: &str) {
    let mut manager = LibraryManager::new();
    
    manager.register_library(create_stdlib_c());
    manager.register_library(create_stdlib_cpp());
    manager.register_library(create_stdlib_rust());
    manager.register_library(create_stdlib_go());
    
    if let Some(lib) = manager.get_library(lib_name) {
        println!("Library: {} v{}", lib.name, lib.version);
        println!("Language: {}", lib.language);
        println!("Description: {}", lib.description);
        println!();
        println!("Functions:");
        for (_, func) in &lib.functions {
            println!("  {}({}) -> {}", 
                func.name,
                func.params.join(", "),
                func.return_type
            );
            println!("    Implementation: {}", func.implementation_language);
        }
    } else {
        eprintln!("Library not found: {}", lib_name);
    }
}

fn create_library(name: &str, language: &str) {
    let lib = library::HioCLibrary::new(
        name.to_string(),
        "1.0.0".to_string(),
        format!("Custom library in {}", language),
        language.to_string(),
    );
    
    let json = lib.export_to_json();
    let filename = format!("{}.hiolib", name);
    
    match fs::write(&filename, json) {
        Ok(_) => {
            println!("Created library {} at {}", name, filename);
        }
        Err(e) => {
            eprintln!("Failed to create library: {}", e);
        }
    }
}

fn start_repl() {
    use std::io::{self, Write};
    
    println!("Hiolang REPL v0.1.0");
    println!("Type 'exit' to quit, 'help' for commands");
    println!();
    
    loop {
        print!("hio> ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let trimmed = input.trim();
                
                if trimmed == "exit" {
                    println!("Goodbye!");
                    break;
                }
                
                if trimmed == "help" {
                    println!("Commands:");
                    println!("  exit  - Exit the REPL");
                    println!("  help  - Show this message");
                    println!("  clear - Clear the screen");
                    continue;
                }
                
                if trimmed.is_empty() {
                    continue;
                }
                
                if trimmed == "clear" {
                    print!("\x1B[2J\x1B[1;1H");
                    continue;
                }
                
                match execute_code(trimmed) {
                    Ok(result) => {
                        if result != ast::Value::Void {
                            println!("=> {}", result.to_string());
                        }
                    }
                    Err(e) => {
                        eprintln!("Error: {}", e);
                    }
                }
            }
            Err(_) => break,
        }
    }
}

impl PartialEq for ast::Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (ast::Value::Number(a), ast::Value::Number(b)) => a == b,
            (ast::Value::Float(a), ast::Value::Float(b)) => (a - b).abs() < f64::EPSILON,
            (ast::Value::String(a), ast::Value::String(b)) => a == b,
            (ast::Value::Boolean(a), ast::Value::Boolean(b)) => a == b,
            (ast::Value::Void, ast::Value::Void) => true,
            _ => false,
        }
    }
}
