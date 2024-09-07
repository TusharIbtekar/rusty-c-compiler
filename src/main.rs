mod code_generator;
mod ir_generator;
mod lexer;
mod optimizer;
mod parser;
mod semantic_analyzer;

use code_generator::generate_output_code;
use ir_generator::generate_ir;
use lexer::lexer;
// use optimizer::optimize_ir;
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

  let mut ir = generate_ir(&ast);
  println!("Intermediate Representation:\n{:?}\n", ir);

  // optimize_ir(&mut ir);
  // println!("Optimized Intermediate Representation:\n{:?}\n", ir);

  let output_code = generate_output_code(&ir);
  println!("Output Code:\n{}", output_code);
}

fn main() {
  let test_code = "
    x = 5;
    y = 10;
    z = x + y;
  ";

  compile(test_code);
}
