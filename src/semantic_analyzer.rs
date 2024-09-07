use std::collections::HashMap;

use crate::parser::AstNode;

pub fn semantic_analysis(ast: &[AstNode]) -> Result<(), String> {
    let mut symbol_table = HashMap::new();

    for node in ast {
        check_node(node, &mut symbol_table)?;
        print!("Symbol Table: {:?}\n", symbol_table);
    }

    Ok(())
}

fn check_node(node: &AstNode, symbol_table: &mut HashMap<String, i32>) -> Result<(), String> {
    match node {
        AstNode::Integer(_) => Ok(()),

        AstNode::Assignment { identifier, value } => {
            check_node(value, symbol_table)?;
            symbol_table.insert(identifier.clone(), 0); // Simplified type checking. Need to update with with proper type
            Ok(())
        }
    }
}
