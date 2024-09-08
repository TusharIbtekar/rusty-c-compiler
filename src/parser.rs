use crate::lexer::Token;

#[derive(Debug, Clone, PartialEq)]
pub enum AstNode {
  Integer(i32),
  BinaryOp {
    left: Box<AstNode>,
    op: Token,
    right: Box<AstNode>,
  },
  Assignment {
    identifier: String,
    value: Box<AstNode>,
  },
  Identifier(String),
  IfElse {
    condition: Box<AstNode>,
    if_branch: Vec<AstNode>,
    else_branch: Option<Vec<AstNode>>,
  },
}

pub fn parse(tokens: &[Token]) -> Vec<AstNode> {
  let mut ast = Vec::new();
  let mut i = 0;

  while i < tokens.len() {
    let (node, new_i) = parse_statement(&tokens[i..]);
    ast.push(node);
    i += new_i;

    // Consume semicolon if present
    if i < tokens.len() && tokens[i] == Token::Semicolon {
      i += 1;
    }
  }

  ast
}

fn parse_statement(tokens: &[Token]) -> (AstNode, usize) {
  match &tokens[0] {
    Token::If => parse_if_else(tokens),
    Token::Identifier(_) => {
      if tokens.len() > 1 && tokens[1] == Token::Equals {
        parse_assignment(tokens)
      } else {
        parse_expression(tokens)
      }
    }
    _ => parse_expression(tokens),
  }
}

fn parse_if_else(tokens: &[Token]) -> (AstNode, usize) {
  let mut i = 1; // Skip 'if'
  let (condition, condition_len) = parse_expression(&tokens[i..]);
  i += condition_len;

  if tokens[i] != Token::LBrace {
    panic!("Expected '{{' after if condition");
  }
  i += 1;

  let (if_branch, if_len) = parse_block(&tokens[i..]);
  i += if_len + 1; // +1 for closing brace

  let else_branch = if i < tokens.len() && tokens[i] == Token::Else {
    i += 1;
    if tokens[i] != Token::LBrace {
      panic!("Expected '{{' after else");
    }
    i += 1;
    let (else_branch, else_len) = parse_block(&tokens[i..]);
    i += else_len + 1; // +1 for closing brace
    Some(else_branch)
  } else {
    None
  };

  (
    AstNode::IfElse {
      condition: Box::new(condition),
      if_branch,
      else_branch,
    },
    i,
  )
}

fn parse_block(tokens: &[Token]) -> (Vec<AstNode>, usize) {
  let mut block = Vec::new();
  let mut i = 0;

  while i < tokens.len() && tokens[i] != Token::RBrace {
    let (node, len) = parse_statement(&tokens[i..]);
    block.push(node);
    i += len;

    if i < tokens.len() && tokens[i] == Token::Semicolon {
      i += 1;
    }
  }

  (block, i)
}

fn parse_assignment(tokens: &[Token]) -> (AstNode, usize) {
  if let Token::Identifier(name) = &tokens[0] {
    let (value, value_len) = parse_expression(&tokens[2..]);
    (
      AstNode::Assignment {
        identifier: name.clone(),
        value: Box::new(value),
      },
      value_len + 2, // +2 for identifier and equals sign
    )
  } else {
    panic!("Expected identifier in assignment");
  }
}

fn parse_expression(tokens: &[Token]) -> (AstNode, usize) {
  let (mut left, mut i) = parse_term(tokens);

  while i < tokens.len() {
    match &tokens[i] {
      Token::Plus
      | Token::Minus
      | Token::DoubleEquals
      | Token::LessThan
      | Token::GreaterThan
      | Token::LessThanEquals
      | Token::GreaterThanEquals => {
        let op = tokens[i].clone();
        i += 1;
        let (right, right_len) = parse_term(&tokens[i..]);
        left = AstNode::BinaryOp {
          left: Box::new(left),
          op,
          right: Box::new(right),
        };
        i += right_len;
      }
      _ => break,
    }
  }

  (left, i)
}

fn parse_term(tokens: &[Token]) -> (AstNode, usize) {
  let (mut left, mut i) = parse_factor(tokens);

  while i < tokens.len() {
    match &tokens[i] {
      Token::Star | Token::Slash => {
        let op = tokens[i].clone();
        i += 1;
        let (right, right_len) = parse_factor(&tokens[i..]);
        left = AstNode::BinaryOp {
          left: Box::new(left),
          op,
          right: Box::new(right),
        };
        i += right_len;
      }
      _ => break,
    }
  }

  (left, i)
}

fn parse_factor(tokens: &[Token]) -> (AstNode, usize) {
  match &tokens[0] {
    Token::Integer(n) => (AstNode::Integer(*n), 1),
    Token::Identifier(name) => (AstNode::Identifier(name.clone()), 1),
    Token::LParen => {
      let (expr, len) = parse_expression(&tokens[1..]);
      if tokens[len + 1] != Token::RParen {
        panic!("Unmatched parentheses");
      }
      (expr, len + 2)
    }
    _ => panic!("Unexpected token in factor: {:?}", tokens[0]),
  }
}
