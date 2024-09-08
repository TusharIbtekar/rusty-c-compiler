use crate::ir_generator::IRInstruction;

pub fn generate_output_code(ir: &[IRInstruction]) -> String {
  let mut output = String::new();
  let mut label_counter = 0;

  output += ".data\n\n";

  output += ".bss\n";
  output += "    stack_pointer DWORD ?\n\n";

  output += ".code\n";
  output += "global main\n\n";

  output += "main PROC\n";

  output += "    push ebp\n";
  output += "    mov ebp, esp\n";
  output += "    sub esp, 256  ; Allocate 256 bytes for local variables\n\n";

  for (i, instruction) in ir.iter().enumerate() {
    match instruction {
      IRInstruction::LoadConstant(n) => {
        output += &format!("    mov eax, {}\n", n);
        output += "    push eax\n";
      }
      IRInstruction::LoadVariable(var) => {
        output += &format!("    mov eax, [ebp+{}]\n", get_variable_offset(var));
        output += "    push eax\n";
      }
      IRInstruction::Add => {
        output += "    pop ebx\n";
        output += "    pop eax\n";
        output += "    add eax, ebx\n";
        output += "    push eax\n";
      }
      IRInstruction::Subtract => {
        output += "    pop ebx\n";
        output += "    pop eax\n";
        output += "    sub eax, ebx\n";
        output += "    push eax\n";
      }
      IRInstruction::Multiply => {
        output += "    pop ebx\n";
        output += "    pop eax\n";
        output += "    imul eax, ebx\n";
        output += "    push eax\n";
      }
      IRInstruction::Divide => {
        output += "    pop ebx\n";
        output += "    pop eax\n";
        output += "    cdq\n";
        output += "    idiv ebx\n";
        output += "    push eax\n";
      }
      IRInstruction::Store(var) => {
        output += "    pop eax\n";
        output += &format!("    mov [ebp+{}], eax\n", get_variable_offset(var));
      }
      IRInstruction::Compare(op) => {
        output += "    pop ebx\n";
        output += "    pop eax\n";
        output += "    cmp eax, ebx\n";
        match op.as_str() {
          "==" => output += "    sete al\n",
          "<" => output += "    setl al\n",
          ">" => output += "    setg al\n",
          "<=" => output += "    setle al\n",
          ">=" => output += "    setge al\n",
          _ => panic!("Invalid comparison operator"),
        }
        output += "    movzx eax, al\n";
        output += "    push eax\n";
      }
      IRInstruction::JumpIfFalse(label) => {
        output += "    pop eax\n";
        output += "    test eax, eax\n";
        output += &format!("    je L{}:\n", label);
        label_counter = label_counter.max(*label + 1);
      }
      IRInstruction::Jump(label) => {
        output += &format!("    jmp L{}:\n", label);
        label_counter = label_counter.max(*label + 1);
      }
    }
    if label_counter > 0 {
      output += &format!("L{}:\n", label_counter);
    }
  }

  output += "\n    mov eax, 0  ; Exit code 0\n";
  output += "    mov esp, ebp\n";
  output += "    pop ebp\n";
  output += "    ret\n";

  output += "main ENDP\n";
  output += "END main\n";

  output
}

fn get_variable_offset(var: &str) -> String {
  format!("{}", (var.chars().next().unwrap() as u32) * 8) // Simplified
}
