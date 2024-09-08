use std::collections::HashMap;

use crate::lexer::Token;
use crate::parser::AstNode;

pub fn semantic_analysis(ast: &[AstNode]) -> Result<(), String> {
  let mut symbol_table = HashMap::new();

  for node in ast {
    check_node(node, &mut symbol_table)?;
  }

  Ok(())
}

fn check_node(node: &AstNode, symbol_table: &mut HashMap<String, i32>) -> Result<(), String> {
  match node {
    AstNode::Integer(_) => Ok(()),
    AstNode::Identifier(name) => {
      if symbol_table.contains_key(name) {
        Ok(())
      } else {
        Err(format!("Variable {} not declared", name))
      }
    }
    AstNode::BinaryOp { left, op, right } => {
      check_node(left, symbol_table)?;
      check_node(right, symbol_table)?;
      match op {
        Token::Plus
        | Token::Minus
        | Token::Star
        | Token::Slash
        | Token::DoubleEquals
        | Token::LessThan
        | Token::GreaterThan
        | Token::LessThanEquals
        | Token::GreaterThanEquals => Ok(()),
        _ => Err(format!("Invalid binary operator: {:?}", op)),
      }
    }
    AstNode::Assignment { identifier, value } => {
      check_node(value, symbol_table)?;
      symbol_table.insert(identifier.clone(), 0);
      Ok(())
    }
    AstNode::IfElse {
      condition,
      if_branch,
      else_branch,
    } => {
      check_node(condition, symbol_table)?;
      for node in if_branch {
        check_node(node, symbol_table)?;
      }
      if let Some(else_nodes) = else_branch {
        for node in else_nodes {
          check_node(node, symbol_table)?;
        }
      }
      Ok(())
    }
  }
}
