mod lexer;
mod parser;

use lexer::lexer;
use parser::parse;

fn compile(input: &str) {
    let tokens = lexer(input);
    println!("Tokens: {:?}", tokens);

    let ast = parse(&tokens);
    println!("Abstract Syntax Tree:\n{:?}\n", ast);
}

fn main() {
    let test_code = "
        x = 5;
        y = 10;
    ";

    compile(test_code);
}
