#[derive(Debug, Clone, PartialEq)]

pub enum Token {
  Integer(i32),
  Identifier(String),
  Plus,
  Minus,
  Star,
  Slash,
  LParen,
  RParen,
  If,
  Else,
  LBrace,
  RBrace,
  Semicolon,
  Equals,
  DoubleEquals,
  LessThan,
  GreaterThan,
  LessThanEquals,
  GreaterThanEquals,
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
        let mut identifier = String::new();
        while let Some(&elem) = chars.peek() {
          if elem.is_alphanumeric() {
            identifier.push(elem);
            chars.next();
          } else {
            break;
          }
        }
        match identifier.as_str() {
          "if" => tokens.push(Token::If),
          "else" => tokens.push(Token::Else),
          _ => tokens.push(Token::Identifier(identifier)),
        }
      }
      '+' => {
        chars.next();
        tokens.push(Token::Plus);
      }
      '-' => {
        chars.next();
        tokens.push(Token::Minus);
      }
      '*' => {
        chars.next();
        tokens.push(Token::Star);
      }
      '/' => {
        chars.next();
        tokens.push(Token::Slash);
      }
      '=' => {
        chars.next();
        if chars.peek() == Some(&'=') {
          chars.next();
          tokens.push(Token::DoubleEquals);
        } else {
          tokens.push(Token::Equals);
        }
      }
      '<' => {
        chars.next();
        if chars.peek() == Some(&'=') {
          chars.next();
          tokens.push(Token::LessThanEquals);
        } else {
          tokens.push(Token::LessThan);
        }
      }
      '>' => {
        chars.next();
        if chars.peek() == Some(&'=') {
          chars.next();
          tokens.push(Token::GreaterThanEquals);
        } else {
          tokens.push(Token::GreaterThan);
        }
      }
      '(' => {
        chars.next();
        tokens.push(Token::LParen);
      }
      ')' => {
        chars.next();
        tokens.push(Token::RParen);
      }
      '{' => {
        chars.next();
        tokens.push(Token::LBrace);
      }
      '}' => {
        chars.next();
        tokens.push(Token::RBrace);
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
