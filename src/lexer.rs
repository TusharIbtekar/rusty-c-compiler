#[derive(Debug, PartialEq)]

pub enum Token {
    Integer(i32),
    Equals,
    Semicolon,
}

pub fn lexer(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    // check for the Tokens
    while let Some(&elem) = chars.peek() {
        match elem {
            '0'..='9' => {
                let mut num = 0;
                while let Some(&elem) = chars.peek() {
                    if elem.is_digit(10) {
                        num = num * 10 + elem.to_digit(10).unwrap() as i32;
                        chars.next();
                    } else {
                        break;
                    }
                }
                tokens.push(Token::Integer(num));
            }
            'a'..='z' | 'A'..='Z' => {
                let mut ident = String::new();
                while let Some(&elem) = chars.peek() {
                    if elem.is_alphanumeric() {
                        ident.push(elem);
                        chars.next();
                    } else {
                        break;
                    }
                }
            }
            '=' => {
                chars.next();
                tokens.push(Token::Equals);
            }
            ';' => {
                chars.next();
                tokens.push(Token::Semicolon);
            }
            ' ' | '\n' | '\t' => {
                chars.next();
            }
            _ => panic!("Unexpected character: {}", elem),
        }
    }
    tokens
}
