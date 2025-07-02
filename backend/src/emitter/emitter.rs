use crate::asm::*;

pub struct CodeEmitter;

impl CodeEmitter {
    pub fn new() -> Self {
        Self
    }

    pub fn emit(&self, program: AsmProgram) -> String {
        let mut output = self.emit_function(&program.function_definition);
        output.push_str("\n.section .note.GNU-stack,\"\",@progbits\n");
        output
    }

    fn emit_function(&self, function: &AsmFunction) -> String {
        let mut output = String::new();

        output.push_str(&format!(".globl {}\n", function.identifier));
        output.push_str(&format!("{}:\n", function.identifier));
        output.push_str("  pushq %rbp\n");
        output.push_str("  movq %rsp, %rbp\n");

        for instr in &function.instructions {
            match instr {
                AsmInstruction::Ret => {
                    output.push_str("  movq %rbp, %rsp\n");
                    output.push_str("  popq %rbp\n");
                    output.push_str("  ret\n");
                }
                _ => {
                    output.push_str(&self.emit_instruction(instr));
                }
            }
        }

        output
    }

    fn emit_instruction(&self, instr: &AsmInstruction) -> String {
        match instr {
            AsmInstruction::Mov { src, dst } => {
                format!(
                    "  movl {}, {}\n",
                    self.emit_operand(src),
                    self.emit_operand(dst)
                )
            }
            AsmInstruction::Unary { unary_operator, operand } => {
                let op_str = match unary_operator {
                    AsmUnaryOperator::Neg => "negl",
                    AsmUnaryOperator::Not => "notl",
                };
                format!("  {} {}\n", op_str, self.emit_operand(operand))
            }
            AsmInstruction::AllocateStack(size) => {
                format!("  subq ${}, %rsp\n", size)
            }
            AsmInstruction::Ret => {
                String::new()
            }
            AsmInstruction::Binary { binary_operator, operand1, operand2 } => {
                format!(
                    "  {} {}, {}\n",
                    self.emit_operator(binary_operator),
                    self.emit_operand(operand1),
                    self.emit_operand(operand2)
                )
            }
            AsmInstruction::Cdq => {
                format!("  cdq\n")
            }
            AsmInstruction::Idiv { operand } => {
                format!("  idivl {}\n", self.emit_operand(operand))
            }
        }
    }

    fn emit_operand(&self, operand: &AsmOperand) -> String {
        match operand {
            AsmOperand::Reg(reg) => match reg {
                AsmReg::AX => "%eax",
                AsmReg::R10 => "%r10d",
                AsmReg::DX => "%edx",
                AsmReg::R11 => "%r11d"
            }.to_string(),
            AsmOperand::Stack(offset) => format!("{}(%rbp)", offset),
            AsmOperand::Imm(value) => format!("${}", value),
            AsmOperand::Pseudo(_) => panic!("Pseudo operand should not exist at emission"),
        }
    }

    fn emit_operator(&self, operator: &AsmBinaryOperator) -> String {
        match operator {
            AsmBinaryOperator::Sub => "subl",
            AsmBinaryOperator::Mult => "imull",
            AsmBinaryOperator::Add => "addl",
        }.to_string()
    }
}
