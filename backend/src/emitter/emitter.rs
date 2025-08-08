use super::emit::*;
use crate::asm::*;
use constructors::constructors;
use match_format::emit_instruction;
use zawarudo::zawarudo;

#[constructors]
pub struct CodeEmitter;

impl CodeEmitter {
    #[zawarudo(label = "Code Emitter")]
    pub fn emit(&self, program: AsmProgram) -> String {
        let mut output = String::new();

        for function in program.top_level() {
            output.push_str(&self.emit_top_level(function));
            output.push('\n');
        }

        output.push_str(".section .note.GNU-stack,\"\",@progbits\n");

        output
    }

    fn emit_top_level(&self, top_level: &AsmTopLevel) -> String {
        match top_level {
            AsmTopLevel::Function { identifier, global, instructions, stack_size: _ } => {
                let mut out = String::new();
                
                if *global {
                    out.push_str(&format!(".globl {}\n", identifier));
                }

                out.push_str(".text\n");

                out.push_str(&format!("{}:\n", identifier));

                out.push_str("    pushq %rbp\n");
                out.push_str("    movq %rsp, %rbp\n");

                for instr in instructions {
                    out.push_str(&self.emit_instruction(instr));
                }
                out
            }

            AsmTopLevel::StaticVariable { identifier, global, init } => {
                let mut out = String::new();

                if *global {
                    out.push_str(&format!(".globl {}\n", identifier));
                }

                if init.parse::<i32>() == Ok(0) {
                    out.push_str(".bss\n");
                    out.push_str(".align 4\n");
                    out.push_str(&format!("{}:\n", identifier));
                    out.push_str(".zero 4\n");
                } else {
                    out.push_str(".data\n");
                    out.push_str(".align 4\n");
                    out.push_str(&format!("{}:\n", identifier));
                    out.push_str(&format!(".long {}\n", init));
                }
                out
            }
        }

    }

    fn emit_instruction(&self, instr: &AsmInstruction) -> String {
        emit_instruction!(instr, {
            AsmInstruction::Mov { src, dst } => "movl {}, {}", [src, dst],
            AsmInstruction::Unary { unary_operator, operand } => "{} {}", [unary_operator, operand],
            AsmInstruction::Binary { binary_operator, src, dst } => "{} {}, {}", [binary_operator, src, dst],
            AsmInstruction::Idiv { operand } => "idivl {}", [operand],
            AsmInstruction::Cdq => "cdq", [],
            AsmInstruction::AllocateStack(int) => "subq ${}, %rsp", [int],
            AsmInstruction::DeallocateStack(int) => "addq ${}, %rsp", [int],
            AsmInstruction::Push(operand) => "pushq {}", [operand.to_8byte()],
            AsmInstruction::Call(label) => "call {}@PLT", [label],
            AsmInstruction::Ret => {
                "movq %rbp, %rsp", [],
                "popq %rbp", [],
                "ret", [],
            },
            AsmInstruction::Cmp { operand1, operand2 } => "cmpl {}, {}", [operand1, operand2],
            AsmInstruction::Jmp(label) => "jmp .L{}", [label],
            AsmInstruction::JmpCC { cond, label } => "j{} .L{}", [cond, label],
            AsmInstruction::SetCC { cond, operand } => "set{} {}", [cond, operand.to_1byte()],
            AsmInstruction::Label(label) => "\t.L{}:", [label],
        })
    }
}