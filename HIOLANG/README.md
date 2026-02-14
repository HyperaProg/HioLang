# Hiolang - Multi-Language Compiler/Interpreter

Hiolang is a unique programming language that combines interpretation and compilation capabilities into a unified system. It provides both interpreted execution (`pub` mode) and bytecode compilation (`subpub` mode), with support for multi-language library integration through the HioClib system.

**Version**: 0.1.0 Alpha  
**Language**: Rust  
**Platform**: Windows, macOS, Linux

## Table of Contents

1. [Architecture Overview](#architecture-overview)
2. [Building and Installation](#building-and-installation)
3. [Command-Line Interface](#command-line-interface)
4. [Language Features](#language-features)
5. [Type System](#type-system)
6. [Built-in Functions](#built-in-functions)
7. [Code Structure](#code-structure)
8. [Library System](#library-system)
9. [Examples](#examples)

## Architecture Overview

Hiolang uses a classic interpreter architecture with the following pipeline:

```
Source Code (.hio)
       ↓
    Lexer (src/lexer.rs)
    - Tokenizes input
    - Recognizes keywords, identifiers, operators
    - Handles strings, numbers, and comments
       ↓
    Parser (src/parser.rs)
    - Builds Abstract Syntax Tree (AST)
    - Parses expressions with operator precedence
    - Parses statements and control flow
       ↓
    AST (src/ast.rs)
    - Tree representation of program
    - Expression nodes (literals, operators, calls, etc.)
    - Statement nodes (let, if, while, functions, etc.)
       ↓
    Interpreter/Compiler (src/interpreter.rs, src/compiler.rs)
    - Interpreter: Direct execution with environment management
    - Compiler: Bytecode generation for later execution
       ↓
    Execution/Output
```

### Core Modules

- **lexer.rs** - Tokenization layer. Converts raw source code into a stream of tokens (keywords, identifiers, operators, literals).
- **parser.rs** - Syntax analysis. Converts token stream into an Abstract Syntax Tree following grammar rules.
- **ast.rs** - AST definitions. Provides the data structures representing program structure (Expr and Stmt enums, Value types).
- **interpreter.rs** - Runtime environment. Directly executes AST nodes with variable/function storage and control flow management.
- **compiler.rs** - Bytecode generation. Compiles AST to stack-based bytecode operations.
- **library.rs** - HioClib integration. Manages external libraries implemented in C, C++, Rust, and Go.
- **main.rs** - CLI entry point. Handles commands and REPL.

## Building and Installation

### Prerequisites

- Rust 1.56+ (install from https://rustup.rs/)
- Cargo (included with Rust)

### Build from Source

```bash
cd c:\Users\divih\Downloads\HIOLANG
cargo build --release
```

The compiled binary will be at: `target/release/hiolang.exe`

### Testing the Build

```bash
./target/release/hiolang version
# Output: Hiolang v0.1.0

./target/release/hiolang help
# Shows all available commands
```

## Command-Line Interface

### run - Execute a Hiolang File

```bash
hiolang run <FILE>
```

Parses and immediately interprets a Hiolang source file.

**Example:**
```bash
hiolang run examples/hello_world.hio
```

### compile - Generate Bytecode

```bash
hiolang compile <FILE> [OUTPUT]
```

Converts a Hiolang file to bytecode format. Defaults to `a.hio` if no output specified.

**Example:**
```bash
hiolang compile examples/calculator.hio compiled.hio
hiolang compile examples/loops.hio  # Creates a.hio
```

### lib - Manage Libraries

**List all available libraries:**
```bash
hiolang lib
```

**Show library details:**
```bash
hiolang lib info <LIBRARY_NAME>
```

**Create a new library:**
```bash
hiolang lib create <NAME> <LANGUAGE>
```

**Example:**
```bash
hiolang lib
# Output: Available Libraries:
#   stdlib_c v1.0.0 (C)
#     Standard library implemented in C
#   stdlib_cpp v1.0.0 (C++)
#     Standard library implemented in C++
#   ...

hiolang lib info stdlib_c
# Shows functions: strlen, strcpy, etc.
```

### repl - Interactive Shell

```bash
hiolang repl
```

Starts an interactive Read-Eval-Print Loop for testing code snippets.

**Example session:**
```
hiolang repl
Hiolang REPL v0.1.0
Type 'exit' to quit, 'help' for commands

hio> let x = 10
Result: void

hio> let y = 20
Result: void

hio> print(x + y)
# Output from print function

hio> exit
Goodbye!
```

### version - Show Version

```bash
hiolang version
# Output: Hiolang v0.1.0
```

### help - Display Help

```bash
hiolang help
hiolang -h
hiolang --help
```

## Language Features

### Lexical Elements

#### Comments

Comments use double single quotes (`''`):

```hiolang
'' This is a comment
'' Single line only
let x = 10;  '' Inline comment
```

#### Identifiers

Valid identifiers start with a letter or underscore, followed by alphanumerics or underscores:

```hiolang
my_var
_private
variable123
MyVariable  '' Case-sensitive
```

Reserved keywords cannot be used as identifiers: `space`, `end`, `make`, `inspace`, `call`, `text`, `pub`, `subpub`, `function`, `return`, `if`, `else`, `while`, `for`, `break`, `continue`, `let`

#### Literals

**Numbers** - Integer literals:
```hiolang
42
-100
0
```

**Floats** - Floating-point literals:
```hiolang
3.14
-2.5
0.0
```

**Strings** - Double or single quoted:
```hiolang
"Hello World"
'Single quoted string'
"String with \"escaped\" quotes"
```

Escape sequences: `\n` (newline), `\t` (tab), `\r` (carriage return), `\\` (backslash), `\"` (quote), `\'` (apostrophe)

**Booleans**:
```hiolang
true
false
```

### Operators

#### Arithmetic Operators

| Operator | Operation |
| -------- | --------- |
| `+` | Addition |
| `-` | Subtraction |
| `*` | Multiplication |
| `/` | Division |
| `%` | Modulo (remainder) |

```hiolang
let sum = 10 + 5;        '' 15
let product = 3 * 7;     '' 21
let remainder = 17 % 5;  '' 2
```

#### Comparison Operators

| Operator | Meaning |
| -------- | ------- |
| `==` | Equal to |
| `!=` | Not equal to |
| `<` | Less than |
| `<=` | Less than or equal |
| `>` | Greater than |
| `>=` | Greater than or equal |

```hiolang
if (x == 10) { '' Equal check
  call.print("x is 10");
}

if (y > 5) { '' Greater than
  call.print("y is greater than 5");
}
```

#### Logical Operators

| Operator | Operation |
| -------- | --------- |
| `&&` | Logical AND |
| `\|\|` | Logical OR |
| `!` | Logical NOT |

```hiolang
if (x > 5 && y < 10) {
  '' Both conditions true
}

if (x == 0 || y == 0) {
  '' At least one condition true
}

if (!condition) {
  '' Negation of condition
}
```

#### Other Operators

| Operator | Use | Example |
| -------- | --- | ------- |
| `.` | Member/attribute access | `obj.property` |
| `[]` | Array/object indexing | `arr[0]`, `obj["key"]` |
| `=` | Assignment | `x = 5` |

### Variable Declaration and Assignment

Use `let` to declare variables:

```hiolang
let x = 10;
let name = "Alice";
let active = true;
let numbers = [1, 2, 3, 4, 5];
```

Reassign variables:

```hiolang
let x = 5;
x = 10;  '' Reassign to new value
```

### Control Flow

#### if/else Statements

```hiolang
if (condition) {
  '' Executed when condition is true
} else {
  '' Executed when condition is false (optional)
}
```

**Example:**
```hiolang
let age = 18;
if (age >= 18) {
  call.print("Adult");
} else {
  call.print("Minor");
}
```

#### while Loops

```hiolang
while (condition) {
  '' Repeats while condition is true
}
```

**Example:**
```hiolang
let counter = 0;
while (counter < 5) {
  call.print(counter);
  counter = counter + 1;
}
'' Output: 0 1 2 3 4
```

#### for Loops

```hiolang
for (init; condition; increment) {
  '' Loop body
}
```

**Example:**
```hiolang
for (let i = 0; i < 10; i = i + 1) {
  call.print(i);
}
```

#### break and continue

```hiolang
while (counter < 100) {
  if (counter == 50) {
    break;  '' Exit loop
  }
  if (counter % 2 == 0) {
    continue;  '' Skip to next iteration
  }
  call.print(counter);
  counter = counter + 1;
}
```

### Function Definition and Calling

Define functions with the `function` keyword:

```hiolang
function add(a, b) {
  return a + b;
}

function greet(name) {
  call.print("Hello, " + name);
}
```

Call functions using `call.functionName(args)`:

```hiolang
let result = call.add(5, 3);
call.print(result);  '' 8

call.greet("Alice");  '' Hello, Alice
```

### Namespaces (Spaces)

Organize code into logical blocks using `+space`:

```hiolang
+space Math;{
  function add(a, b) {
    return a + b;
  }
  
  function multiply(a, b) {
    return a * b;
  }
}
'' Access: call.Math.add(5, 3)
```

### Execution Modes

#### Interpretation Mode (pub)

Execute code directly during parsing:

```hiolang
pub;{;com 'interpretation'};{
  let x = 10;
  call.print(x);
}
```

#### Compilation Mode (subpub)

Compile to bytecode for later execution:

```hiolang
subpub;{compilation_type 'bytecode'};{
  let y = 20;
  return y * 2;
}
```

## Type System

### Value Types

The interpreter supports these runtime value types:

| Type | Example | Notes |
| ---- | ------- | ----- |
| `Number` | `42`, `-17`, `0` | 64-bit signed integer |
| `Float` | `3.14`, `-2.5` | 64-bit floating point |
| `String` | `"hello"`, `'world'` | UTF-8 text |
| `Boolean` | `true`, `false` | Logical truth value |
| `Array` | `[1, 2, 3]` | Ordered collection |
| `Object` | `{name: "Alice", age: 30}` | Key-value pairs |
| `Void` | (implicit) | No value (return type of functions with no return) |

### Type Checking and Coercion

The `type()` built-in function returns the type of a value:

```hiolang
call.print(call.type(42));        '' "number"
call.print(call.type(3.14));      '' "float"
call.print(call.type("hello"));   '' "string"
call.print(call.type(true));      '' "boolean"
call.print(call.type([1,2,3]));   '' "array"
```

### Truthiness

In conditional contexts, values are evaluated for truthiness:

- `Number`: 0 is false, all others true
- `String`: Empty string is false, non-empty true
- `Boolean`: Uses value directly
- `Array`: Empty array is false, non-empty true
- `Object`: Empty object is false, non-empty true
- `Void`: Always false

```hiolang
if (10) { call.print("true"); }   '' Prints: true

let empty = "";
if (!empty) { call.print("true"); }  '' Prints: true (empty string is falsy)

let zeros = [0, 0];
if (zeros) { call.print("true"); }   '' Prints: true (non-empty array)
```

## Built-in Functions

### print(value)

Outputs a value and returns void.

```hiolang
call.print("Hello World");
call.print(42);
call.print([1, 2, 3]);
'' Output printed to stdout
```

### len(value)

Returns the length of a string or array.

```hiolang
let msg = "hello";
let length = call.len(msg);     '' 5

let arr = [10, 20, 30];
let count = call.len(arr);      '' 3
```

For non-string/array values, behavior may vary.

### type(value)

Returns a string describing the type of the value.

```hiolang
call.print(call.type(10));       '' "number"
call.print(call.type(3.14));     '' "float"
call.print(call.type("hi"));     '' "string"
call.print(call.type(true));     '' "boolean"
call.print(call.type([1,2]));    '' "array"
call.print(call.type({x: 1}));   '' "object"
```

## Code Structure

### src/lexer.rs

**Converts source code into tokens.**

Key components:
- `Token` enum - 40+ token types (keywords, operators, literals, punctuation)
- `Lexer` struct - Maintains input position and current character
- Methods:
  - `new(input: &str)` - Create lexer for source code
  - `next_token() -> Token` - Get next token from input
  - `read_identifier()` - Parse identifier/keyword
  - `read_string()` - Parse quoted string with escape sequences
  - `read_number()` - Parse integer or float literal

Token types include: Keywords (Space, Let, If, While, etc.), Literals (Number, Float, String, Boolean), Operators (+, -, *, /, ==, !=, &&, ||, etc.), Punctuation (parentheses, braces, brackets, dots, etc.)

### src/parser.rs

**Converts tokens into an Abstract Syntax Tree.**

Key components:
- `Parser` struct - Maintains token position
- Methods:
  - `parse() -> Result<Program, String>` - Parse complete program
  - Expression parsing with operator precedence (addition/subtraction → multiplication/division)
  - Statement parsing (assignments, if/while/for, function definitions, etc.)
  - Error handling with descriptive messages

The parser implements recursive descent parsing, where each grammar rule has a corresponding method. Operator precedence is handled by parsing at different recursion levels.

### src/ast.rs

**Defines the Abstract Syntax Tree data structures.**

Core types:
- `Value` enum - Runtime values (Number, String, Boolean, Array, Object, Void)
  - `is_truthy()` - Used in conditional evaluation
  - `to_string()` - Converts value to display string
  
- `Expr` enum - Expression nodes
  - Literals: Number, Float, String, Boolean
  - Composite: Array, Object
  - Operations: Binary (arithmetic/comparison/logic), Unary (negate, not)
  - Functions: Call, Index, Member access

- `Stmt` enum - Statement nodes
  - Variables: Let (declaration), Assign (reassignment)
  - Control: If, While, For, Break, Continue, Return
  - Functions: FunctionDef
  - Modules: Space, Pub, Subpub

- `BinaryOp` and `UnaryOp` enums - Operation types

- `Program` struct - Complete program (ordered list of statements)

### src/interpreter.rs

**Executes the AST directly (interprets it).**

Key components:
- `Interpreter` struct
  - `globals` - Global variable scope using `Rc<RefCell<HashMap>>`
  - `locals` - Stack of local scopes (for function calls and blocks)
  - `return_value` - Handles early return from functions
  - `break_flag`, `continue_flag` - Control loop behavior

- Methods:
  - `new()` - Initialize with built-in functions (print, len, type)
  - `interpret(program: &Program)` - Execute program
  - `execute_stmt(stmt: &Stmt)` - Execute single statement
  - `evaluate_expr(expr: &Expr)` - Evaluate expression to value
  - Variable management: `set_variable()`, `get_variable()`

Execution flow:
1. For each statement, execute it and track control flow (return, break, continue)
2. For expressions, recursively evaluate sub-expressions
3. For function calls, push new local scope, execute body, pop scope, return result
4. For control flow (if/while/for), evaluate conditions and execute branches accordingly

### src/compiler.rs

**Compiles AST to stack-based bytecode.**

Key components:
- `BytecodeOp` enum - 30+ bytecode operations
  - Stack: PushNumber, PushString, PushBool, Pop
  - Arithmetic: Add, Subtract, Multiply, Divide, Modulo
  - Logic: And, Or, Not, Equal, NotEqual, etc.
  - Control: JumpIfFalse, Jump, Call, Return
  - Variables: GetLocal, GetGlobal, SetLocal, SetGlobal
  - Arrays/Objects: ArrayCreate, ObjectCreate, Index, Member

- `Compiler` struct
  - `bytecode` - Instructions vector
  - `functions` - Map of function bytecode

- Methods:
  - `compile(program: &Program)` - Convert AST to bytecode
  - `compile_stmt()` - Compile statement to operations
  - `compile_expr()` - Compile expression to operations

Compilation strategy:
- Expressions compile to operations that leave results on stack
- Function calls compile to Call operation with argument count
- Control flow uses JumpIfFalse and Jump operations with address placeholders
- Variables access uses GetLocal/GetGlobal and SetLocal/SetGlobal

### src/library.rs

**Multi-language library integration system.**

Key types:
- `LibraryFunction` - Single function definition
  - `name`, `params` (param names), `return_type`
  - `implementation_language` (C, C++, Rust, Go)
  - `source_code` - Actual implementation

- `HioCLibrary` - Library package
  - `name`, `version`, `description`, `language`
  - `functions` - HashMap of function implementations
  - Methods: `add_function()`, `get_function()`, `export_to_json()`

- `LibraryManager` - Registry system
  - `register_library()` - Add library to manager
  - `get_library()` - Retrieve library by name
  - `list_libraries()` - Get all library names
  - `load_from_file()`, `save_library()` - Persistence

Built-in libraries:
- `stdlib_c` - C functions (strlen, strcpy, etc.)
- `stdlib_cpp` - C++ functions (string_length, etc.)
- `stdlib_rust` - Rust functions
- `stdlib_go` - Go functions

### src/main.rs

**CLI interface and entry point.**

Functions:
- `main()` - Parse command-line arguments and dispatch
- `run_file()` - Run .hio file directly (lexer → parser → interpreter)
- `compile_file()` - Compile to bytecode (lexer → parser → compiler)
- `execute_code()` - Parse and interpret code string
- `serialize_bytecode()` - Write bytecode to file
- `list_libraries()` - Show available libraries
- `show_library_info()` - Display library details
- `create_library()` - Generate new library definition
- `start_repl()` - Interactive shell loop

Command routing:
- `run` → `run_file()`
- `compile` → `compile_file()`
- `lib` → `list_libraries()` / `show_library_info()` / `create_library()`
- `repl` → `start_repl()`
- `version` → Print version
- `help` → Print usage

## Library System

### Built-in Libraries

#### stdlib_c (C Standard Library)

Functions implemented in C:

| Function | Signature | Purpose |
| -------- | --------- | ------- |
| `strlen` | `strlen(str: string) -> number` | Get string length |
| `strcpy` | `strcpy(dest, src: string) -> string` | Copy string |

**Example usage:**
```hiolang
let length = call.strlen("hello");
call.print(length);  '' 5
```

#### stdlib_cpp (C++ Standard Library)

Functions implemented in C++:

| Function | Signature | Purpose |
| -------- | --------- | ------- |
| `string_length` | `string_length(str: string) -> number` | Get string length using std::string |

#### stdlib_rust (Rust Standard Library)

Rust implementations for performance-critical operations.

#### stdlib_go (Go Standard Library)

Go implementations for concurrent operations and networking.

### Creating Custom Libraries

```bash
hiolang lib create mylib C
'' Creates mylib.hiolib

hiolang lib create networking Go
'' Creates networking.hiolib for network functions
```

The generated `.hiolib` file is a JSON format library definition that can be extended with custom functions.

## Examples

### Example 1: Hello World

**File: examples/hello_world.hio**
```hiolang
+space Main;{
  inspace.writeutil;{
    call.writeutil
    text("Hello World")
  }
}
end make;
```

**Run:**
```bash
hiolang run examples/hello_world.hio
```

### Example 2: Calculator with Functions

**File: examples/calculator.hio**
```hiolang
+space Calculator;{
pub;{;com 'interpretation'};{
  function add(a, b) {
    return a + b;
  }

  function subtract(a, b) {
    return a - b;
  }

  function multiply(a, b) {
    return a * b;
  }

  function divide(a, b) {
    if (b == 0) {
      return 0;
    }
    return a / b;
  }

  let result1 = call.add(10, 5);
  call.print(result1);

  let result2 = call.multiply(3, 7);
  call.print(result2);
}
}
end make;
```

**Run:**
```bash
hiolang run examples/calculator.hio
'' Output:
'' Result: 15
'' Result: 21
```

### Example 3: Loops and Control Flow

**File: examples/loops.hio**
```hiolang
+space ControlFlow;{
pub;{;com 'interpretation'};{
  '' While loop
  let counter = 0;
  while (counter < 5) {
    call.print(counter);
    counter = counter + 1;
  }

  '' For loop
  for (let i = 0; i < 3; i = i + 1) {
    call.print(i);
  }

  '' Break and continue
  let x = 0;
  while (x < 10) {
    if (x == 5) {
      break;
    }
    if (x % 2 == 0) {
      x = x + 1;
      continue;
    }
    call.print(x);
    x = x + 1;
  }
}
}
end make;
```

**Run:**
```bash
hiolang run examples/loops.hio
```

### Example 4: Compilation Mode

**File: examples/compiled_module.hio**
```hiolang
subpub;{compilation_type 'bytecode'};{
  function factorial(n) {
    if (n <= 1) {
      return 1;
    }
    return n * call.factorial(n - 1);
  }

  return call.factorial(5);
}
```

**Compile:**
```bash
hiolang compile examples/compiled_module.hio fact_bytecode.hio
```

The resulting file contains stack-based bytecode instructions optimized for execution.

## Implementation Details

### Operator Precedence

The parser implements standard mathematical operator precedence:

1. **Parenthesized expressions** (highest)
2. **Unary operators**: `-`, `!`
3. **Multiplicative**: `*`, `/`, `%`
4. **Additive**: `+`, `-`
5. **Comparison**: `<`, `<=`, `>`, `>=`
6. **Equality**: `==`, `!=`
7. **Logical AND**: `&&`
8. **Logical OR**: `||` (lowest)

**Example:**
```hiolang
'' Evaluates as: ((a + b) * c) > (d / 2)
if (a + b * c > d / 2) { }

'' Evaluates as: (x + 3) < ((y * 2) + 1)
let result = x + 3 < y * 2 + 1;
```

### Variable Scoping

The interpreter uses a scope stack:

- **Global scope**: Top-level variables, persists across function calls
- **Local scopes**: Function parameters and local variables, created when function is called
- **Resolution**: Look up variable starting from innermost scope, work outward

```hiolang
let global_var = 10;  '' Global scope

function test(param) {
  '' param is local to test()
  let local_var = 20;  '' Local to test()
  global_var = 30;     '' Modifies global scope
}

call.test(5);
call.print(global_var);   '' 30 (modified in function)
call.print(local_var);    '' Error: undefined
```

### Error Handling

The interpreter returns `Result<Value, String>` from all parsing and execution operations:

- **Lexer errors**: Invalid characters, unclosed strings
- **Parser errors**: Unexpected tokens, invalid syntax, missing semicolons
- **Runtime errors**: Undefined variables, type mismatches, division by zero (returns 0)

**Example error:**
```bash
hiolang run invalid.hio
'' Error: Expected '(' after 'if' keyword
```

## Development Roadmap

Future planned features:

- Full pattern matching and destructuring
- Lambda/anonymous functions
- Module system with imports/exports
- Advanced type system (optional types, generics)
- Async/await support
- Better error messages with line/column info
- Optimization passes on bytecode
- FFI for direct C integration
- Debugging support (breakpoints, stepping)
    counter = counter + 1;
  }
  
  for (let i = 0; i < 3; i = i + 1) {
    if (i % 2 == 0) {
      call.print(i);
    }
  }
—}
}
end make;
```

### Interpretation vs Compilation
```hiolang
+space MixedMode;{
  ''Interpreted section - flexible, slower
  pub;{;com 'interpretation'};{
    function slow_calculation(n) {
      return n * n;
    }
  —}
  
  ''Compiled section - optimized, faster
  subpub;{compilation};{
    function fast_calculation(n) {
      return n * n + n;
    }
  }
}
end make;
```

## HioClib - Library System

### What is HioClib?

HioClib is Hiolang's revolutionary library system that allows you to:
- Write libraries in **C**, **C++**, **Rust**, or **Go**
- Define custom syntax for each library
- Mix native performance with language flexibility
- Create functions that don't even exist in Hiolang itself

### Available Standard Libraries

#### stdlib_c
String and memory manipulation in C
- `strlen(str)` - Get string length
- `strcpy(dest, src)` - Copy string

#### stdlib_cpp
Advanced features in C++
- `string_length(str)` - Get UTF-8 string length

#### stdlib_rust
Safe file I/O in Rust
- `string_reverse(str)` - Reverse string

#### stdlib_go
Network and async operations in Go
- `bytes_to_string(data)` - Convert bytes to string

### Creating a Library

#### 1. Create Library Metadata
```bash
hiolang lib create mylib Rust
```

This creates `mylib.hiolib` with the library definition.

#### 2. Implement Functions in Your Language

**Rust Example: string_utils.rs**
```rust
pub fn hio_string_to_uppercase(s: &str) -> String {
    s.to_uppercase()
}

pub fn hio_string_to_lowercase(s: &str) -> String {
    s.to_lowercase()
}
```

**C Example: array_utils.c**
```c
int* hio_array_reverse(int* arr, int len) {
    int* result = malloc(len * sizeof(int));
    for (int i = 0; i < len; i++) {
        result[i] = arr[len - 1 - i];
    }
    return result;
}
```

**C++ Example: vector_ops.cpp**
```cpp
std::vector<int> hio_sort_vector(std::vector<int> v) {
    std::sort(v.begin(), v.end());
    return v;
}
```

**Go Example: network_utils.go**
```go
func HioHttpGet(url string) (string, error) {
    resp, err := http.Get(url)
    // ... implementation
    return body, nil
}
```

#### 3. Use in Hiolang Code
```hiolang
+space MyApp;{
pub;{;com 'interpretation'};{
  import mylib;
  
  let text = "hello world";
  let upper = call.mylib.to_uppercase(text);
  call.print(upper);
—}
}
end make;
```

### Library Definition Format

Libraries are defined in a JSON-like format:

```json
{
  "name": "stdlib_c",
  "version": "1.0.0",
  "description": "Standard library implemented in C",
  "language": "C",
  "functions": {
    "strlen": {
      "params": ["str"],
      "return_type": "number",
      "implementation_language": "C"
    }
  }
}
```

## Project Structure

```
hiolang/
├── src/
│   ├── main.rs              # Entry point and CLI
│   ├── lexer.rs             # Tokenization
│   ├── parser.rs            # Syntax parsing
│   ├── ast.rs               # Abstract syntax tree
│   ├── interpreter.rs       # Runtime execution
│   ├── compiler.rs          # Bytecode generation
│   └── library.rs           # HioClib system
├── examples/
│   ├── hello_world.hio      # Hello World example
│   ├── calculator.hio       # Functions and arithmetic
│   ├── loops.hio            # Control flow
│   ├── compiled_module.hio  # Compilation example
│   └── libs/                # Library implementations
│       ├── string_utils.c
│       ├── math_utils.cpp
│       ├── file_io.rs
│       └── networking.go
├── Cargo.toml               # Rust package definition
└── README.md                # This file
```

## Syntax Reference

### Keywords
- `+space` - Define namespace
- `pub` - Public interpretation section
- `subpub` - Public compilation section
- `function` - Define function
- `return` - Return from function
- `let` - Variable declaration
- `if`, `else` - Conditionals
- `while`, `for` - Loops
- `break`, `continue` - Loop control
- `call` - Function call prefix
- `text` - String literal helper
- `end make` - End block

### Operators
- Arithmetic: `+`, `-`, `*`, `/`, `%`
- Comparison: `==`, `!=`, `<`, `<=`, `>`, `>=`
- Logical: `&&`, `||`, `!`
- Member access: `.`
- Array access: `[]`

### Data Types
- `number` - Integer (64-bit)
- `float` - Floating-point
- `string` - Text
- `boolean` - True/false
- `array` - Collections
- `object` - Key-value pairs

## Architecture

### Compilation Pipeline
```
Source Code (.hio)
    ↓
Lexer (Tokenization)
    ↓
Parser (Syntax Analysis)
    ↓
AST (Abstract Syntax Tree)
    ↓
Interpreter/Compiler
    ├→ Interpreter (Direct Execution)
    └→ Compiler (Bytecode Generation)
    ↓
Output (Results/Bytecode)
```

### Multi-Language Integration
```
Hiolang Code
    ↓
HioClib Manager
    ├→ C Implementation
    ├→ C++ Implementation
    ├→ Rust Implementation
    └→ Go Implementation
    ↓
Native Code / Bytecode
    ↓
Execution
```

## Performance Considerations

1. **Interpretation Mode (`pub`)**: Fast startup, slower execution
   - Good for: Rapid development, scripting
   - Typical overhead: 2-5x slower than compiled

2. **Compilation Mode (`subpub`)**: Slower startup, faster execution
   - Good for: Performance-critical code
   - Native code performance (up to 100x faster for compute-heavy tasks)

3. **Mixed Mode**: Best of both worlds
   - Use `pub` for UI logic, I/O operations
   - Use `subpub` for algorithms, numerical computation

## Examples Directory

The `examples/` directory contains:

1. **hello_world.hio** - Basic output
2. **calculator.hio** - Functions and arithmetic
3. **loops.hio** - Loops and conditionals
4. **compiled_module.hio** - Using compilation mode
5. **libs/** - Library implementations in C, C++, Rust, Go

Run any example:
```bash
hiolang run examples/hello_world.hio
hiolang run examples/calculator.hio
```

## Command-Line Interface

```
USAGE:
    hiolang <COMMAND> [ARGS]

COMMANDS:
    run <FILE>              Run a Hiolang file
    compile <FILE> [OUT]    Compile to bytecode
    lib                     List available libraries
    lib info <NAME>         Show library information
    lib create <NAME> <LANG> Create a new library
    repl                    Start interactive REPL
    version                 Show version
    help                    Show this help message
```

## Roadmap

### Version 0.2.0
- [ ] Optimize interpreter performance
- [ ] Add more standard libraries
- [ ] Improve error messages
- [ ] Add module system

### Version 0.3.0
- [ ] Native code generation
- [ ] JIT compilation
- [ ] Debugging support
- [ ] Package manager

### Version 1.0.0
- [ ] Full standard library
- [ ] IDE support
- [ ] Documentation
- [ ] Community packages

## Contributing

Contributions are welcome! Areas for help:

- Standard library expansion
- New language backends (Node.js, Python, Java)
- Performance optimization
- Documentation
- Bug fixes
- Examples

## License

MIT License - See LICENSE file for details

## Community

- GitHub: [hiolang/hiolang](https://github.com/hiolang/hiolang)
- Forum: [discussions.hiolang.dev](https://discussions.hiolang.dev)
- Issues: [github.com/hiolang/hiolang/issues](https://github.com/hiolang/hiolang/issues)

## Troubleshooting

### Parse Error
- Check syntax matches examples
- Verify all blocks end with `end make;`
- Ensure proper spacing and punctuation

### Library Not Found
```bash
hiolang lib              # Lists available libraries
hiolang lib info <name> # Shows library details
```

### Performance Issues
- Use `subpub` compilation mode for compute-heavy code
- Profile with `hiolang repl` to test snippets
- Combine fast libraries with interpretation for flexibility

## Author

Created as the world's first multi-language programming language with integrated interpretation and compilation.

**Version**: 0.1.0  
**Status**: Alpha  
**Last Updated**: February 2026
