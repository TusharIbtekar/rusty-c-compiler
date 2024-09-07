mod lexer;
mod parser;
mod semantic_analyzer;

use lexer::lexer;
use parser::parse;
use semantic_analyzer::semantic_analysis;

fn compile(input: &str) {
    let tokens = lexer(input);
    println!("Tokens: {:?}", tokens);

    let ast = parse(&tokens);
    println!("Abstract Syntax Tree:\n{:?}\n", ast);

    match semantic_analysis(&ast) {
        Ok(()) => println!("Semantic analysis passed.\n"),
        Err(e) => {
            println!("Semantic analysis failed: {}\n", e);
            return;
        }
    }
}

fn main() {
    let test_code = "
        x = 5;
        y = 10;
    ";

    compile(test_code);
}
