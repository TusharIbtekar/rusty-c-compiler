# Rusty C Compiler

This project is a simple C compiler written in Rust. It demonstrates the basic stages of compilation, including lexical analysis, parsing, semantic analysis, intermediate code generation, and final code generation.

To get an overview of how a compiler works, [read here](https://medium.com/@ibtekar/how-a-compiler-works-a-simple-breakdown-a498edc39762)

## Features

- Lexical analysis (tokenization)
- Parsing (AST generation)
- Semantic analysis
- Intermediate Representation (IR) generation
- x86-64 assembly code generation
- Support for basic arithmetic operations
- Support for if-else statements and comparisons

## Project Structure

The compiler is organized into several modules:

- `lexer`: Tokenizes the input code.
- `parser`: Generates an Abstract Syntax Tree (AST) from the tokens.
- `semantic_analyzer`: Performs basic semantic checks on the AST.
- `ir_generator`: Generates Intermediate Representation (IR) from the AST.
- `code_generator`: Produces x86-64 assembly code from the IR.

## Usage

### Prerequisites

Ensure you have [Rust](https://www.rust-lang.org/tools/install) installed on your system.

### Steps

1. Clone the repository:

   ```bash
   git clone git@github.com:TusharIbtekar/rusty-c-compiler.git
   ```

2. Navigate to the project directory:

   ```bash
   cd rusty-c-compiler
   ```

3. Run the compiler:

   ```bash
   cargo run
   ```

### Example Code

Here's a simple example of code that the compiler can process:

```c
x = 5;
y = 10;
z = x + y;
if (x < y) {
    result = x * 2;
} else {
    result = y / 2;
}
if (result <= 15) {
    final = result + 5;
} else {
    final = result - 5;
}
```

## Compiler Stages

### 1. Lexical Analysis

The lexer (`lexer.rs`) breaks down the input code into a series of tokens, recognizing elements such as:

- Identifiers
- Integer literals
- Arithmetic operators (+, -, \*, /)
- Comparison operators (==, <, >, <=, >=)
- Parentheses and braces
- Keywords (`if`, `else`)

To extend the lexer, add new token types to the `Token` enum and update the `lexer` function to recognize new patterns.

### 2. Parsing

The parser (`parser.rs`) takes the tokens produced by the lexer and constructs an Abstract Syntax Tree (AST). It supports:

- Arithmetic expressions
- Assignment statements
- If-else statements

To add new language constructs, extend the `AstNode` enum and update the parsing functions accordingly.

### 3. Semantic Analysis

The semantic analyzer (`semantic_analyzer.rs`) performs basic checks on the AST, such as ensuring variables are declared before use. To add more semantic checks, modify the `semantic_analysis` function.

### 4. Intermediate Representation (IR) Generation

The IR generator (`ir_generator.rs`) converts the AST into a linear representation closer to assembly code. To support new language features, add new IR instructions to the `IRInstruction` enum and update the `generate_ir` function.

### 5. Code Generation

The code generator (`code_generator.rs`) produces x86-64 assembly code from the IR. It includes:

- Basic arithmetic operations
- Variable assignment
- Conditional branching

To generate code for new features, update the `generate_output_code` function.

## Extending the Compiler

To add new features:

1. Add new token types to the lexer if necessary.
2. Extend the parser to recognize new syntax structures.
3. Update the semantic analyzer to handle new constructs.
4. Add new IR instructions, if required.
5. Implement code generation for new features.

## Testing

Currently, the project uses a hardcoded test in the `main` function.

## Limitations

- Limited operations and control structures
- No support for functions or complex data types
- Simplified error handling
- No optimizations
- Simplified MASM generator

## Future Improvements

Some potential areas for improvement include:

- Function definitions and calls
- More complex data types (arrays, structs)
- Type system implementation
- Optimizations in IR and code generation
- Better error reporting and recovery

## Contributing

Contributions to improve the compiler are welcome! Please feel free to submit issues or pull requests.
