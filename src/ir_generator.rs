use crate::{lexer::Token, parser::AstNode};

#[derive(Debug, PartialEq)]

pub enum IRInstruction {
  LoadConstant(i32),
  LoadVariable(String),
  Add,
  Subtract,
  Multiply,
  Divide,
  Store(String),
  Compare(String), // "==", "<", ">", "<=", ">="
  JumpIfFalse(usize),
  Jump(usize),
}

pub fn generate_ir(ast: &[AstNode]) -> Vec<IRInstruction> {
  let mut ir = Vec::new();
  let mut label_counter = 0;

  for node in ast {
    ir.extend(generate_ir_node(node, &mut label_counter));
  }

  ir
}

fn generate_ir_node(node: &AstNode, label_counter: &mut usize) -> Vec<IRInstruction> {
  match node {
    AstNode::Integer(n) => vec![IRInstruction::LoadConstant(*n)],
    AstNode::Identifier(name) => vec![IRInstruction::LoadVariable(name.clone())],
    AstNode::BinaryOp { left, op, right } => {
      let mut ir = generate_ir_node(left, label_counter);
      ir.extend(generate_ir_node(right, label_counter));
      ir.push(match op {
        Token::Plus => IRInstruction::Add,
        Token::Minus => IRInstruction::Subtract,
        Token::Star => IRInstruction::Multiply,
        Token::Slash => IRInstruction::Divide,
        Token::DoubleEquals => IRInstruction::Compare("==".to_string()),
        Token::LessThan => IRInstruction::Compare("<".to_string()),
        Token::GreaterThan => IRInstruction::Compare(">".to_string()),
        Token::LessThanEquals => IRInstruction::Compare("<=".to_string()),
        Token::GreaterThanEquals => IRInstruction::Compare(">=".to_string()),
        _ => panic!("Invalid binary operator"),
      });
      ir
    }
    AstNode::Assignment { identifier, value } => {
      let mut ir = generate_ir_node(value, label_counter);
      ir.push(IRInstruction::Store(identifier.clone()));
      ir
    }
    AstNode::IfElse {
      condition,
      if_branch,
      else_branch,
    } => {
      let mut ir = generate_ir_node(condition, label_counter);
      let else_label = *label_counter;
      *label_counter += 1;
      let end_label = *label_counter;
      *label_counter += 1;

      ir.push(IRInstruction::JumpIfFalse(else_label));

      for node in if_branch {
        ir.extend(generate_ir_node(node, label_counter));
      }

      if else_branch.is_some() {
        ir.push(IRInstruction::Jump(end_label));
      }

      ir.push(IRInstruction::Jump(end_label));
      ir.push(IRInstruction::JumpIfFalse(else_label));

      for node in if_branch {
        ir.extend(generate_ir_node(node, label_counter));
      }

      if let Some(else_nodes) = else_branch {
        ir.push(IRInstruction::Jump(end_label));
        ir.push(IRInstruction::JumpIfFalse(else_label));
        for node in else_nodes {
          ir.extend(generate_ir_node(node, label_counter));
        }
      }

      ir.push(IRInstruction::JumpIfFalse(end_label));
      ir
    }
  }
}
