# Hiolang Project Structure

```
c:\Users\divih\Downloads\HIOLANG\
├── Cargo.toml              # Rust package manifest
├── README.md               # Main documentation
├── LANGUAGE_SPEC.md        # Complete language specification
├── HIOLIB_GUIDE.md         # HioClib library system guide
├── .gitignore              # Git ignore rules
│
├── src/                    # Source code
│   ├── main.rs             # Entry point, CLI, REPL
│   ├── lexer.rs            # Tokenizer/Lexer
│   ├── parser.rs           # Parser (tokens → AST)
│   ├── ast.rs              # Abstract Syntax Tree definitions
│   ├── interpreter.rs      # Runtime interpreter
│   ├── compiler.rs         # Bytecode compiler
│   └── library.rs          # HioClib system
│
├── examples/               # Example programs
│   ├── hello_world.hio     # Basic "Hello World"
│   ├── calculator.hio      # Functions and arithmetic
│   ├── loops.hio           # Control flow examples
│   ├── compiled_module.hio # Compilation mode example
│   ├── comprehensive.hio   # Full-featured example
│   │
│   └── libs/               # Library implementations
│       ├── string_utils.c          # C string utilities
│       ├── math_utils.cpp          # C++ math functions
│       ├── file_io.rs              # Rust file operations
│       └── networking.go           # Go networking
│
└── target/                 # Build output (after cargo build)
    └── release/
        └── hiolang         # Compiled binary
```

## Quick Start

### 1. Building (requires Rust/Cargo)
```bash
cd c:\Users\divih\Downloads\HIOLANG
cargo build --release
```

### 2. Running Examples
```bash
# Once built, run examples
./target/release/hiolang run examples/hello_world.hio
./target/release/hiolang run examples/calculator.hio
./target/release/hiolang run examples/loops.hio
./target/release/hiolang run examples/comprehensive.hio
```

### 3. Interactive REPL
```bash
./target/release/hiolang repl
```

### 4. Compile to Bytecode
```bash
./target/release/hiolang compile examples/calculator.hio output.hio
```

### 5. View Available Libraries
```bash
./target/release/hiolang lib
./target/release/hiolang lib info stdlib_c
./target/release/hiolang lib info stdlib_cpp
./target/release/hiolang lib info stdlib_rust
./target/release/hiolang lib info stdlib_go
```

## File Descriptions

### Core Implementation Files

**main.rs**
- CLI argument parsing
- Command handling (run, compile, lib, repl, version)
- REPL implementation
- Library management UI

**lexer.rs**
- Tokenization (text → tokens)
- Keyword recognition
- String/number parsing
- Comment handling

**parser.rs**
- Syntax analysis (tokens → AST)
- Expression parsing with precedence
- Statement parsing
- Error reporting

**ast.rs**
- AST node definitions (Expr, Stmt, Program)
- Value types (Number, String, Boolean, Array, Object, etc.)
- Type system representation

**interpreter.rs**
- Runtime execution engine
- Variable scoping with local/global namespaces
- Built-in function implementation
- Expression evaluation

**compiler.rs**
- Bytecode generation
- Optimization for compiled mode
- Function compilation
- Type checking during compilation

**library.rs**
- HioClib library management
- Library function registration
- Multi-language library support (C, C++, Rust, Go)
- Library loading and execution

### Documentation Files

**README.md**
- Project overview
- Installation instructions
- Basic examples
- Feature overview
- Troubleshooting guide

**LANGUAGE_SPEC.md**
- Complete language specification
- Syntax reference
- Data types and operations
- All keywords and operators
- EBNF grammar definition

**HIOLIB_GUIDE.md**
- HioClib system documentation
- Creating libraries in each language
- Library examples
- Best practices
- Performance tips

### Example Programs

**hello_world.hio**
- Basic output example
- Demonstrates writeutil functionality

**calculator.hio**
- Function definitions
- Multiple operations
- Print statements

**loops.hio**
- For loops
- While loops
- Conditionals

**compiled_module.hio**
- Subpub compilation mode
- Recursive function example

**comprehensive.hio**
- Factorial calculation
- Prime number checking
- Array operations
- Full feature demonstration

### Library Examples

**string_utils.c**
- String operations in C
- String conversion functions
- Memory management

**math_utils.cpp**
- Mathematical algorithms in C++
- Prime checking
- GCD/LCM calculations
- Factorial computation

**file_io.rs**
- Safe file operations in Rust
- Read/write/append operations
- Directory listing

**networking.go**
- HTTP requests in Go
- Timestamp operations
- Sleep/timing functions

## Development Workflow

### 1. Making Changes to Language
1. Edit relevant source file (lexer.rs, parser.rs, etc.)
2. Run `cargo build` to check for errors
3. Run `cargo test` for tests (if available)
4. Create example in `examples/` to test feature

### 2. Adding New Library
1. Create `.rs`, `.c`, `.cpp`, or `.go` file in `examples/libs/`
2. Implement functions with `hio_` prefix
3. Update `library.rs` to register functions
4. Test with example program

### 3. Extending Language Features
1. Add tokens to `lexer.rs` if needed
2. Add AST nodes to `ast.rs`
3. Add parser rules to `parser.rs`
4. Add interpreter logic to `interpreter.rs`
5. Add compiler support to `compiler.rs`

## Project Statistics

- **Lines of Code (src/)**
  - lexer.rs: ~350 lines
  - parser.rs: ~550 lines
  - ast.rs: ~130 lines
  - interpreter.rs: ~400 lines
  - compiler.rs: ~400 lines
  - library.rs: ~300 lines
  - main.rs: ~300 lines

- **Total**: ~2,500+ lines of Rust

- **Documentation**: ~3,000+ lines
- **Examples**: ~200 lines of Hiolang code
- **Libraries**: ~400 lines across C, C++, Rust, Go

## Dependencies

The project uses only Rust standard library (no external dependencies by design):
- No need for external parsing libraries
- All functionality implemented from scratch
- Lightweight and portable

## Platform Support

- ✅ Windows (PowerShell, CMD)
- ✅ macOS (Terminal, Bash, Zsh)  
- ✅ Linux (Bash, Fish, Zsh)

Requires: Rust 1.56+ and Cargo

## Next Steps

1. **Build the project**: `cargo build --release`
2. **Run examples**: See Quick Start above
3. **Read documentation**: Start with README.md
4. **Explore examples**: Check examples/ directory
5. **Create your own**: Write a .hio file and run it

## Support

For questions or issues:
1. Check LANGUAGE_SPEC.md for syntax reference
2. Review HIOLIB_GUIDE.md for library system
3. Study examples in examples/ directory
4. Use REPL for quick testing

---

**Hiolang v0.1.0 Alpha**  
**Created**: February 2026  
**Status**: Fully Implemented Core Features
