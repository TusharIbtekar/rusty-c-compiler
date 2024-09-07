use crate::lexer::Token;

#[derive(Debug, PartialEq)]
pub enum AstNode {
    Integer(i32),
    Assignment {
        identifier: String,
        value: Box<AstNode>,
    },
}

pub fn parse(tokens: &[Token]) -> Vec<AstNode> {
    let mut ast = Vec::new();
    let mut i = 0;
    // Tokens: [Identifier("x"), Equals, Integer(5), Semicolon, Identifier("y"), Equals, Integer(10), Semicolon]
    while i < tokens.len() {
        match &tokens[i] {
            Token::Identifier(name) => {
                if i + 2 < tokens.len() && tokens[i + 1] == Token::Equals {
                    let (value, new_i) = parse_factor(&tokens[i + 2..]);
                    ast.push(AstNode::Assignment {
                        identifier: name.clone(),
                        value: Box::new(value),
                    });
                    i += new_i + 3; // Skip identifier, equals, and expression
                } else {
                    let (node, new_i) = parse_factor(&tokens[i..]);
                    ast.push(node);
                    i += new_i;
                }
            }
            _ => {
                let (node, new_i) = parse_factor(&tokens[i..]);
                ast.push(node);
                i += new_i;
            }
        }

        // Consume semicolon if present
        if i < tokens.len() && tokens[i] == Token::Semicolon {
            i += 1;
        }
    }

    ast
}

fn parse_factor(tokens: &[Token]) -> (AstNode, usize) {
    print!("Tokens: {:?}\n", tokens);
    match &tokens[0] {
        Token::Integer(n) => (AstNode::Integer(*n), 1),

        _ => panic!("Unexpected token in factor"),
    }
}
