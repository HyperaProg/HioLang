# HioClib - Hiolang Library System Documentation

## Overview

HioClib (Hiolang C Library System) is a revolutionary approach to library development that allows you to:

1. **Write libraries in any supported language** (C, C++, Rust, Go)
2. **Define custom syntax** for each library
3. **Seamlessly integrate** with Hiolang code
4. **Achieve native performance** without sacrificing flexibility

## Supported Languages

### C
- **Best for**: Low-level operations, memory management
- **Performance**: Excellent
- **Complexity**: Medium
- **Use cases**: String manipulation, system calls, file I/O

### C++
- **Best for**: Complex algorithms, object-oriented code
- **Performance**: Excellent
- **Complexity**: High
- **Use cases**: Mathematical operations, data structures, graphics

### Rust
- **Best for**: Safe, concurrent code
- **Performance**: Excellent
- **Complexity**: Medium-High
- **Use cases**: File I/O, threading, async operations

### Go
- **Best for**: Networking, concurrent tasks
- **Performance**: Good
- **Complexity**: Low-Medium
- **Use cases**: HTTP requests, network protocols, goroutine-based work

## Library Structure

```
mylib/
├── mylib.hiolib          # Library definition
├── src/
│   ├── c/
│   │   ├── string_ops.c
│   │   └── Makefile
│   ├── cpp/
│   │   ├── math_ops.cpp
│   │   └── CMakeLists.txt
│   ├── rust/
│   │   └── lib.rs
│   └── go/
│       ├── main.go
│       └── go.mod
└── include/              # Header files for C/C++
    └── mylib.h
```

## Creating a Library

### Step 1: Initialize Library
```bash
hiolang lib create mylib Rust
```

### Step 2: Define Functions

**Example: String utilities in C**

`src/c/string_ops.c`:
```c
#include "../include/mylib.h"
#include <string.h>
#include <ctype.h>

char* hio_uppercase(const char* str) {
    char* result = malloc(strlen(str) + 1);
    for (int i = 0; str[i]; i++) {
        result[i] = toupper(str[i]);
    }
    result[strlen(str)] = '\0';
    return result;
}
```

**Example: Math utilities in C++**

`src/cpp/math_ops.cpp`:
```cpp
#include "mylib.h"
#include <cmath>
#include <vector>

std::vector<int> hio_primes(int limit) {
    std::vector<int> result;
    for (int i = 2; i <= limit; i++) {
        bool is_prime = true;
        for (int j = 2; j * j <= i; j++) {
            if (i % j == 0) {
                is_prime = false;
                break;
            }
        }
        if (is_prime) result.push_back(i);
    }
    return result;
}
```

**Example: File I/O in Rust**

`src/rust/lib.rs`:
```rust
use std::fs;
use std::io;

pub fn hio_read_file(path: &str) -> io::Result<String> {
    fs::read_to_string(path)
}

pub fn hio_write_file(path: &str, content: &str) -> io::Result<()> {
    fs::write(path, content)
}
```

**Example: HTTP in Go**

`src/go/main.go`:
```go
package main

import "net/http"
import "io/ioutil"

func HioHttpGet(url string) (string, error) {
    resp, err := http.Get(url)
    if err != nil {
        return "", err
    }
    defer resp.Body.Close()
    
    body, _ := ioutil.ReadAll(resp.Body)
    return string(body), nil
}
```

### Step 3: Create Library Definition

`mylib.hiolib`:
```json
{
  "name": "mylib",
  "version": "1.0.0",
  "description": "My custom Hiolang library",
  "languages": ["rust", "go"),
  "functions": {
    "read_file": {
      "params": ["path"],
      "return_type": "string",
      "language": "rust"
    },
    "http_get": {
      "params": ["url"],
      "return_type": "string",
      "language": "go"
    }
  }
}
```

### Step 4: Register Library
```bash
hiolang lib install mylib
```

### Step 5: Use in Hiolang

```hiolang
+space MyApp;{
pub;{;com 'interpretation'};{
  import mylib;
  
  let content = call.mylib.read_file("data.txt");
  let response = call.mylib.http_get("https://api.example.com");
  
  call.print(content);
  call.print(response);
—}
}
end make;
```

## Built-in Libraries

### stdlib_c - C Standard Library
String and memory functions in C

**Functions:**
- `strlen(str)` - Get string length
- `strcpy(dest, src)` - Copy string
- `strcat(dest, src)` - Concatenate strings
- `strcmp(str1, str2)` - Compare strings

**Example usage:**
```hiolang
+space StringOps;{
pub;{;com 'interpretation'};{
  import stdlib_c;
  
  let len = call.stdlib_c.strlen("Hello");
  call.print(len);
—}
}
end make;
```

### stdlib_cpp - C++ Advanced Library
Advanced features in C++

**Functions:**
- `string_length(str)` - UTF-8 aware length
- `string_split(str, delim)` - Split string
- `sort_array(arr)` - Sort array
- `vector_median(arr)` - Calculate median

**Example usage:**
```hiolang
+space MathOps;{
pub;{;com 'interpretation'};{
  import stdlib_cpp;
  
  let arr = [3, 1, 4, 1, 5, 9, 2, 6];
  let sorted = call.stdlib_cpp.sort_array(arr);
  let median = call.stdlib_cpp.vector_median(sorted);
  call.print(median);
—}
}
end make;
```

### stdlib_rust - Rust Safe Operations
Safe file I/O and string operations

**Functions:**
- `file_read(path)` - Read file safely
- `file_write(path, content)` - Write file
- `string_reverse(str)` - Reverse string
- `file_append(path, content)` - Append to file

**Example usage:**
```hiolang
+space FileOps;{
pub;{;com 'interpretation'};{
  import stdlib_rust;
  
  let data = call.stdlib_rust.file_read("config.txt");
  call.print(data);
—}
}
end make;
```

### stdlib_go - Go Networking
Network and async operations

**Functions:**
- `http_get(url)` - GET request
- `http_post(url, data)` - POST request
- `get_timestamp()` - Current timestamp
- `sleep(ms)` - Sleep milliseconds

**Example usage:**
```hiolang
+space AsyncOps;{
pub;{;com 'interpretation'};{
  import stdlib_go;
  
  let response = call.stdlib_go.http_get("https://api.example.com");
  let timestamp = call.stdlib_go.get_timestamp();
  
  call.print(response);
  call.print(timestamp);
—}
}
end make;
```

## Custom Syntax in Libraries

Each library can define its own domain-specific language (DSL):

### Example: Mathematical DSL in a library

**Library definition allows custom operators:**
```hiolang
+space MathAdvanced;{
subpub;{compilation};{
  ''Custom syntax: matrix notation
  let matrix = call.math.matrix[[1,2,3], [4,5,6], [7,8,9]];
  let result = call.math.determinant(matrix);
—}
}
end make;
```

### Example: SQL-like syntax in a database library
```hiolang
+space DatabaseOps;{
pub;{;com 'interpretation'};{
  ''SQL-like syntax via library
  let users = call.db.query{
    select: ["id", "name", "email"],
    from: "users",
    where: ["age", ">", 18]
  };
—}
}
end make;
```

## Compilation and Linking

### From C
```bash
gcc -o mylib_c.so -shared src/c/string_ops.c
```

### From C++
```bash
g++ -o mylib_cpp.so -shared -fPIC src/cpp/math_ops.cpp
```

### From Rust
```bash
cargo build --release --crate-type cdylib
```

### From Go
```bash
go build -o mylib_go.so -buildmode=c-shared
```

## Inter-language Communication

Libraries can call functions across languages through Hiolang's type system:

```hiolang
+space Polyglot;{
pub;{;com 'interpretation'};{
  import string_utils_c;  ''C library
  import array_utils_cpp; ''C++ library
  import file_ops_rust;   ''Rust library
  import net_utils_go;    ''Go library
  
  ''All can be called seamlessly
  let text = call.string_utils_c.uppercase("hello");
  let sorted = call.array_utils_cpp.sort([3,1,2]);
  let content = call.file_ops_rust.read_file("data.txt");
  let response = call.net_utils_go.http_get("https://api.example.com");
—}
}
end make;
```

## Performance Tips

1. **Use C for maximum performance** on simple operations
2. **Use C++ for advanced data structures** and algorithms
3. **Use Rust for **FILE I/O and safety-critical operations
4. **Use Go for network operations** and concurrent tasks

## Debugging Libraries

### Enable debug output
```bash
hiolang run --debug program.hio
```

### Check library info
```bash
hiolang lib info mylib
```

### Test library function
```bash
hiolang repl
hio> import mylib;
hio> call.mylib.test_function("arg");
```

## Distribution

### Package library
```bash
hiolang lib package mylib mylib-1.0.0.hiopack
```

### Publish to registry
```bash
hiolang lib publish mylib
```

### Install from registry
```bash
hiolang lib install user/mylib
```

## Security Considerations

1. **Validate input** in C/C++ to prevent buffer overflows
2. **Use Rust** for memory-unsafe operations
3. **Sandbox network operations** in Go
4. **Verify library sources** before installation

## Troubleshooting

### Library not loading
```bash
hiolang lib install mylib
hiolang lib info mylib
```

### Linking errors
- Ensure all dependencies are compiled
- Check architecture compatibility (x86_64, ARM)
- Verify shared object paths

### Type mismatch errors
- Ensure parameter types match library definition
- Check return type conversions
- Enable debug mode for detailed errors

## Examples

See the `examples/libs/` directory for complete implementations in:
- `string_utils.c` - C string operations
- `math_utils.cpp` - C++ mathematical functions
- `file_io.rs` - Rust file operations
- `networking.go` - Go network utilities
