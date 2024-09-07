use crate::parser::AstNode;

#[derive(Debug, PartialEq)]

pub enum IRInstruction {
    LoadConstant(i32),
    Store(String),
}

pub fn generate_ir(ast: &[AstNode]) -> Vec<IRInstruction> {
    let mut ir = Vec::new();
    for node in ast {
        ir.extend(generate_ir_node(node));
    }
    ir
}

pub fn generate_ir_node(node: &AstNode) -> Vec<IRInstruction> {
    match node {
        AstNode::Integer(n) => vec![IRInstruction::LoadConstant(*n)],
        AstNode::Assignment { identifier, value } => {
            let mut ir = generate_ir_node(value);
            ir.push(IRInstruction::Store(identifier.clone()));
            ir
        }
    }
}