use crate::ir_generator::IRInstruction;

pub fn generate_output_code(ir: &[IRInstruction]) -> String {
    let mut output = String::new();
    for (i, instruction) in ir.iter().enumerate() {
        match instruction {
            IRInstruction::LoadConstant(n) => output += &format!("LOAD {}\n", n),
            IRInstruction::Store(var) => output += &format!("STORE {}\n", var),
        }
        output += &format!("L{}:\n", i + 1);
    }
    output
}
