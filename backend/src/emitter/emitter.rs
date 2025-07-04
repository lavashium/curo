use crate::asm::*;
use super::emit::ToAsm;
use match_format::emit_instruction;

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
        output.push_str("    pushq %rbp\n");
        output.push_str("    movq %rsp, %rbp\n");

        for instr in &function.instructions {
            output.push_str(&self.emit_instruction(instr));
        }

        output
    }

    fn emit_instruction(&self, instr: &AsmInstruction) -> String {
        emit_instruction!(instr, {
            AsmInstruction::Mov { src, dst } => "movl {}, {}", [src, dst],
            AsmInstruction::Unary { unary_operator, operand } => "{} {}", [unary_operator, operand],
            AsmInstruction::Binary { binary_operator, src, dst } => "{} {}, {}", [binary_operator, src, dst],
            AsmInstruction::Idiv { operand } => "idivl {}", [operand],
            AsmInstruction::Cdq => "cdq", [],
            AsmInstruction::AllocateStack(int) => "subq ${}, %rsp", [int],
            AsmInstruction::Ret => {
                "movq %rbp, %rsp", [],
                "popq %rbp", [],
                "ret", [],
            },
        })
    }
}
