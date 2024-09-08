use crate::ir_generator::IRInstruction;

pub fn generate_output_code(ir: &[IRInstruction]) -> String {
  let mut output = String::new();
  let mut label_counter = 0;

  output += "    .data\n";

  output += "    .bss\n";
  output += "    .lcomm stack_pointer, 8\n\n";

  output += "    .text\n";
  output += "    .globl main\n\n";
  output += "main:\n";

  output += "    pushq %rbp\n";
  output += "    movq %rsp, %rbp\n";
  output += "    subq $256, %rsp  # Allocate 256 bytes for local variables\n\n";

  for (i, instruction) in ir.iter().enumerate() {
    match instruction {
      IRInstruction::LoadConstant(n) => {
        output += &format!("    movq ${}, %rax\n", n);
        output += "    pushq %rax\n";
      }
      IRInstruction::LoadVariable(var) => {
        output += &format!("    movq {}(%rbp), %rax\n", get_variable_offset(var));
        output += "    pushq %rax\n";
      }
      IRInstruction::Add => {
        output += "    popq %rbx\n";
        output += "    popq %rax\n";
        output += "    addq %rbx, %rax\n";
        output += "    pushq %rax\n";
      }
      IRInstruction::Subtract => {
        output += "    popq %rbx\n";
        output += "    popq %rax\n";
        output += "    subq %rbx, %rax\n";
        output += "    pushq %rax\n";
      }
      IRInstruction::Multiply => {
        output += "    popq %rbx\n";
        output += "    popq %rax\n";
        output += "    imulq %rbx, %rax\n";
        output += "    pushq %rax\n";
      }
      IRInstruction::Divide => {
        output += "    popq %rbx\n";
        output += "    popq %rax\n";
        output += "    cqo\n";
        output += "    idivq %rbx\n";
        output += "    pushq %rax\n";
      }
      IRInstruction::Store(var) => {
        output += "    popq %rax\n";
        output += &format!("    movq %rax, {}(%rbp)\n", get_variable_offset(var));
      }
      IRInstruction::Compare(op) => {
        output += "    popq %rbx\n";
        output += "    popq %rax\n";
        output += "    cmpq %rbx, %rax\n";
        match op.as_str() {
          "==" => output += "    sete %al\n",
          "<" => output += "    setl %al\n",
          ">" => output += "    setg %al\n",
          "<=" => output += "    setle %al\n",
          ">=" => output += "    setge %al\n",
          _ => panic!("Invalid comparison operator"),
        }
        output += "    movzbq %al, %rax\n";
        output += "    pushq %rax\n";
      }
      IRInstruction::JumpIfFalse(label) => {
        output += "    popq %rax\n";
        output += "    testq %rax, %rax\n";
        output += &format!("    je .L{}\n", label);
        label_counter = label_counter.max(*label + 1);
      }
      IRInstruction::Jump(label) => {
        output += &format!("    jmp .L{}\n", label);
        label_counter = label_counter.max(*label + 1);
      }
    }
    if i + 1 == label_counter {
      output += &format!(".L{}:\n", i + 1);
    }
  }

  output += "\n    # Function epilogue\n";
  output += "    movq $0, %rax  # Return 0\n";
  output += "    leave\n";
  output += "    ret\n";

  output
}

fn get_variable_offset(var: &str) -> String {
  format!("-{}(%rbp)", var.chars().next().unwrap() as u32 * 8) // simplified
}
