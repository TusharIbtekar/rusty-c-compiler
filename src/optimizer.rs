use crate::ir_generator::IRInstruction;

pub fn optimize_ir(ir: &mut Vec<IRInstruction>) {
  let mut i = 0;
  while i < ir.len() - 2 {
    match (&ir[i], &ir[i + 1], &ir[i + 2]) {
      (IRInstruction::LoadConstant(a), IRInstruction::LoadConstant(b), IRInstruction::Add) => {
        ir[i] = IRInstruction::LoadConstant(a + b);
        ir.remove(i + 1);
        ir.remove(i + 1);
      }
      _ => i += 1,
    }
  }
}
