# Hiolang Language Specification

## Version: 0.1.0 Alpha

## Table of Contents
1. [Lexical Structure](#lexical-structure)
2. [Data Types](#data-types)
3. [Variables](#variables)
4. [Operators](#operators)
5. [Statements](#statements)
6. [Functions](#functions)
7. [Modules and Namespaces](#modules-and-namespaces)
8. [Execution Modes](#execution-modes)
9. [Built-in Functions](#built-in-functions)
10. [Error Handling](#error-handling)

## Lexical Structure

### Comments
```hiolang
'' Single line comment
'' Can only use double single quotes
```

### Identifiers
- Start with letter or underscore: `[a-zA-Z_]`
- Followed by letters, digits, or underscore: `[a-zA-Z0-9_]*`
- Case-sensitive
- Reserved words cannot be used

### Keywords
```
space, end, make, inspace, call, text, pub, subpub,
function, return, if, else, while, for, break, continue, let
```

### Literals

#### Numbers
```hiolang
42                  '' Integer
-17                 '' Negative integer
0
```

#### Floating-point
```hiolang
3.14                '' Float
-2.5
0.0
```

#### Strings
```hiolang
"Hello World"       '' Double quotes
'Hello'             '' Single quotes (alternative)
```

#### Boolean
```hiolang
true
false
```

### Operators

#### Arithmetic
```
+   Addition
-   Subtraction
*   Multiplication
/   Division
%   Modulo
```

#### Comparison
```
==  Equal
!=  Not equal
<   Less than
<=  Less than or equal
>   Greater than
>=  Greater than or equal
```

#### Logical
```
&&  AND
||  OR
!   NOT
```

#### Other
```
.   Member access
[]  Array/object indexing
=   Assignment
->  Arrow function (future)
```

## Data Types

### Primitive Types

#### Number (Integer)
```hiolang
let age = 25;
let count = -10;
```

#### Float
```hiolang
let pi = 3.14;
let epsilon = 2.718;
```

#### String
```hiolang
let name = "Alice";
let message = "Hello, World!";
let empty = "";
```

#### Boolean
```hiolang
let active = true;
let logged_in = false;
```

#### Void
- Implicit return type
- No explicit void literals

### Composite Types

#### Array
```hiolang
let numbers = [1, 2, 3, 4, 5];
let mixed = [1, "two", 3.0, true];
let empty_arr = [];
let nested = [[1, 2], [3, 4]];
```

#### Object
```hiolang
let person = {
  "name": "Bob",
  "age": 30,
  "active": true
};

let config = {
  "debug": false,
  "version": "1.0.0"
};
```

## Variables

### Declaration
```hiolang
let x = 10;                    '' Declare and initialize
let name = "Alice";            '' String variable
let items = [1, 2, 3];        '' Array variable
```

### Assignment
```hiolang
x = 20;                        '' Reassign existing variable
name = "Bob";
items[0] = 100;               '' Array element assignment
```

### Scoping
```hiolang
+space Outer;{
  let x = 10;                 '' Outer scope
  
  +space Inner;{
    let x = 20;               '' Inner scope shadows outer
    call.print(x);            '' Prints 20
  }
  end make;
  
  call.print(x);              '' Prints 10
}
end make;
```

## Operators

### Operator Precedence (highest to lowest)

1. Primary: `()`, `[]`, `.`
2. Unary: `-`, `!`
3. Multiplicative: `*`, `/`, `%`
4. Additive: `+`, `-`
5. Comparison: `<`, `<=`, `>`, `>=`
6. Equality: `==`, `!=`
7. Logical AND: `&&`
8. Logical OR: `||`

## Statements

### Expression Statements
```hiolang
call.print(42);
x + y;
function_call();
```

### Variable Declaration
```hiolang
let variable = expression;
```

### Assignment
```hiolang
identifier = expression;
```

### If Statement
```hiolang
if (condition) {
  '' Then branch
}

if (condition) {
  '' Then branch
} else {
  '' Else branch
}

if (a == 1) {
  call.print("One");
} else if (a == 2) {
  call.print("Two");
} else {
  call.print("Other");
}
```

### While Statement
```hiolang
while (condition) {
  '' Loop body
  '' Executed while condition is true
}
```

### For Statement
```hiolang
for (initialization; condition; increment) {
  '' Loop body
}

for (let i = 0; i < 10; i = i + 1) {
  call.print(i);
}
```

### Break and Continue
```hiolang
for (let i = 0; i < 10; i = i + 1) {
  if (i == 5) {
    break;                    '' Exit loop
  }
  if (i == 2) {
    continue;                 '' Skip to next iteration
  }
  call.print(i);
}
```

### Return Statement
```hiolang
function greet(name) {
  if (name == "") {
    return "Hello, stranger!";
  }
  return "Hello, " + name + "!";
}
```

### Block Statement
```hiolang
{
  '' Local scope
  let x = 10;
  call.print(x);
}
```

## Functions

### Function Definition
```hiolang
function add(a, b) {
  return a + b;
}

function greet(name) {
  let greeting = "Hello, " + name;
  call.print(greeting);
}

function no_return() {
  call.print("No return value");
}
```

### Function Calls
```hiolang
let result = call.add(5, 3);
call.greet("Alice");
call.no_return();
```

### Parameters and Arguments
```hiolang
'' Function with multiple parameters
function math_op(x, y, z) {
  return x + y * z;
}

'' Calling with arguments
let result = call.math_op(1, 2, 3);
```

### Return Values
```hiolang
'' Multiple return types
function check_value(x) {
  if (x > 0) {
    return "positive";
  } else if (x < 0) {
    return "negative";
  } else {
    return "zero";
  }
}

let status = call.check_value(-5);
```

### Recursion
```hiolang
function factorial(n) {
  if (n <= 1) {
    return 1;
  }
  return n * call.factorial(n - 1);
}

let result = call.factorial(5);  '' Returns 120
```

## Modules and Namespaces

### Space Declaration
```hiolang
+space ModuleName;{
  '' Module code
  let x = 10;
  
  function helper() {
    return x;
  }
}
end make;
```

### Nested Spaces
```hiolang
+space Outer;{
  let x = 10;
  
  +space Inner;{
    let x = 20;  '' Shadows outer x
  }
  end make;
}
end make;
```

### Library Import
```hiolang
+space App;{
pub;{;com 'interpretation'};{
  import stdlib_c;
  import custom_lib;
  
  let len = call.stdlib_c.strlen("Hello");
  let result = call.custom_lib.custom_function();
—}
}
end make;
```

## Execution Modes

### Interpretation Mode (pub)
```hiolang
+space MyApp;{
pub;{;com 'interpretation'};{
  '' This code is interpreted
  '' Flexible but slower
  
  let x = 10;
  call.print(x);
—}
}
end make;
```

**Characteristics:**
- Direct execution
- No compilation overhead
- Slower execution
- Good for development

### Compilation Mode (subpub)
```hiolang
+space MyApp;{
subpub;{compilation};{
  '' This code is compiled
  '' Fast execution
  
  function compute(n) {
    return n * n + n;
  }
  
  let result = call.compute(100);
}
}
end make;
```

**Characteristics:**
- Compiled to bytecode
- Compilation overhead
- Faster execution
- Good for production

### Mixed Mode
```hiolang
+space Hybrid;{
  pub;{;com 'interpretation'};{
    '' Interpreted section
    let data = call.read_input();
  —}
  
  subpub;{compilation};{
    '' Compiled section
    function process(d) {
      return d * 2;
    }
    let result = call.process(data);
  }
}
end make;
```

## Built-in Functions

### I/O Operations

#### print(...)
```hiolang
call.print("Hello");
call.print(42);
call.print([1, 2, 3]);
call.print(true);
```

#### writeutil.text(string)
```hiolang
call.writeutil.text("Direct output");
```

### Type Operations

#### len(value)
```hiolang
let str_len = call.len("Hello");           '' Returns 5
let arr_len = call.len([1, 2, 3]);        '' Returns 3
```

#### type(value)
```hiolang
call.type(42);              '' Returns "number"
call.type(3.14);            '' Returns "float"
call.type("text");          '' Returns "string"
call.type(true);            '' Returns "boolean"
call.type([1, 2]);          '' Returns "array"
call.type({});              '' Returns "object"
```

## Error Handling

### Runtime Errors

#### Division by zero
```hiolang
let result = 10 / 0;        '' Error: Division by zero
```

#### Index out of bounds
```hiolang
let arr = [1, 2, 3];
let item = arr[10];         '' Error: Index out of bounds
```

#### Undefined variable
```hiolang
call.print(undefined_var);  '' Error: Undefined variable
```

### Type Coercion

#### Implicit conversions
```hiolang
let str = "Hello" + 123;    '' Result: "Hello123"
let result = 5 + 3.14;      '' Result: 8.14
```

#### Explicit type checking
```hiolang
let value = 42;
if (call.type(value) == "number") {
  call.print("It's a number");
}
```

## Standard Library Reference

### String Functions
```hiolang
call.len(str)               '' String length
call.type(str)              '' Type checking
```

### Array Functions
```hiolang
call.len(arr)               '' Array length
```

### Input/Output
```hiolang
call.print(value)           '' Print to stdout
call.writeutil.text(str)    '' Raw text output
```

## Example Programs

### Fibonacci Sequence
```hiolang
+space Fibonacci;{
pub;{;com 'interpretation'};{
  function fib(n) {
    if (n <= 1) {
      return n;
    }
    return call.fib(n - 1) + call.fib(n - 2);
  }
  
  for (let i = 0; i < 10; i = i + 1) {
    let result = call.fib(i);
    call.print(result);
  }
—}
}
end make;
```

### Array Operations
```hiolang
+space ArrayOps;{
pub;{;com 'interpretation'};{
  let arr = [5, 2, 8, 1, 9];
  let sum = 0;
  
  for (let i = 0; i < call.len(arr); i = i + 1) {
    sum = sum + arr[i];
  }
  
  call.print(sum);            '' Prints 25
—}
}
end make;
```

### String Processing
```hiolang
+space StringOps;{
pub;{;com 'interpretation'};{
  let text = "Hello World";
  let len = call.len(text);
  
  call.print("Text: " + text);
  call.print("Length: " + len);
  
  if (text == "Hello World") {
    call.print("Match!");
  }
—}
}
end make;
```

## Reserved Identifiers

### Built-in Functions
```
print, len, type, call
```

### Keywords
```
space, end, make, pub, subpub, function, return,
let, if, else, while, for, break, continue
```

## EBNF Grammar

```ebnf
program         = statement* EOF

statement       = space_stmt
                | pub_stmt
                | subpub_stmt
                | func_def
                | let_stmt
                | assignment
                | if_stmt
                | while_stmt
                | for_stmt
                | return_stmt
                | break_stmt
                | continue_stmt
                | expr_stmt
                | block_stmt

space_stmt      = "+space" IDENTIFIER "{" statement* "}" "end" "make" ";"

pub_stmt        = "pub" ";" "{" ";" "com" STRING ";" "{" statement* "}" "—"

subpub_stmt     = "subpub" ";" "{" "compilation" "}" "{" statement* "}"

func_def        = "function" IDENTIFIER "(" param_list ")" "{" statement* "}"

param_list      = (IDENTIFIER ("," IDENTIFIER)*)? 

let_stmt        = "let" IDENTIFIER "=" expr ";"

assignment      = IDENTIFIER "=" expr ";"

if_stmt         = "if" "(" expr ")" block ("else" block)?

while_stmt      = "while" "(" expr ")" block

for_stmt        = "for" "(" (let_stmt | "")
                  (expr ";" | ";")
                  (expr)? ")" block

return_stmt     = "return" expr? ";"

break_stmt      = "break" ";"

continue_stmt   = "continue" ";"

expr_stmt       = expr ";"

block_stmt      = "{" statement* "}"

block           = "{" statement* "}"

expr            = lor_expr

lor_expr        = land_expr ("||" land_expr)*

land_expr       = eq_expr ("&&" eq_expr)*

eq_expr         = rel_expr (("==" | "!=") rel_expr)*

rel_expr        = add_expr (("<" | "<=" | ">" | ">=") add_expr)*

add_expr        = mul_expr (("+" | "-") mul_expr)*

mul_expr        = unary_expr (("*" | "/" | "%") unary_expr)*

unary_expr      = ("!" | "-")? postfix_expr

postfix_expr    = primary_expr ("(" args ")" | "[" expr "]" | "." IDENTIFIER)*

primary_expr    = NUMBER
                | FLOAT
                | STRING
                | "true"
                | "false"
                | IDENTIFIER
                | "(" expr ")"
                | "[" (expr ("," expr)*)? "]"
                | "{" (STRING ":" expr ("," STRING ":" expr)*)? "}"

args            = (expr ("," expr)*)?
```

---

**Last Updated**: February 2026  
**Status**: Alpha Release  
**Specification Version**: 0.1.0
