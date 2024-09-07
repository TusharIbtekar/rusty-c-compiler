mod lexer;

use lexer::lexer;

fn compile(input: &str) {
    let tokens = lexer(input);
    println!("Tokens: {:?}", tokens);
}

fn main() {
    let test_code = "
        x = 5;
    ";

    compile(test_code);
}
