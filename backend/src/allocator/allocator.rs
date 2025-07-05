use crate::asm::*;
use accessors::accessors;
use std::collections::HashMap;

#[accessors]
pub struct AsmAllocator {
    next_offset: i32,
    stack_map: HashMap<String, i32>,
}

impl AsmAllocator {
    pub fn new() -> Self {
        Self {
            next_offset: -4,
            stack_map: HashMap::new(),
        }
    }

    pub fn allocate(mut self, program: AsmProgram) -> (AsmProgram, i32) {
        let function = self.visit_function(program.get_function_definition());
        let final_offset = self.next_offset;
        (AsmProgram::new(function), -final_offset)
    }

    fn visit_function(&mut self, function: AsmFunction) -> AsmFunction {
        let instructions = function
            .get_instructions()
            .into_iter()
            .map(|instr| self.visit_instruction(instr))
            .collect();

        AsmFunction::new(function.get_identifier(), instructions)
    }

    fn visit_instruction(&mut self, instr: AsmInstruction) -> AsmInstruction {
        match instr {
            AsmInstruction::Mov { src, dst } => {
                AsmInstruction::new_mov(self.replace_operand(src), self.replace_operand(dst))
            }

            AsmInstruction::Unary { unary_operator, operand } => {
                AsmInstruction::new_unary(unary_operator, self.replace_operand(operand))
            }

            AsmInstruction::Ret => AsmInstruction::new_ret(),

            AsmInstruction::Cdq => AsmInstruction::new_cdq(),

            AsmInstruction::AllocateStack(amount) => {
                AsmInstruction::new_allocate_stack(amount)
            }

            AsmInstruction::Binary { binary_operator, src, dst } => {
                AsmInstruction::new_binary(
                    binary_operator,
                    self.replace_operand(src),
                    self.replace_operand(dst),
                )
            }

            AsmInstruction::Idiv { operand } => {
                AsmInstruction::new_idiv(self.replace_operand(operand))
            }

            AsmInstruction::Cmp { operand1, operand2 } => {
                AsmInstruction::new_cmp(self.replace_operand(operand1), self.replace_operand(operand2))
            }

            AsmInstruction::SetCC { cond, operand } => {
                AsmInstruction::new_set_c_c(cond, self.replace_operand(operand))
            }

            AsmInstruction::Jmp(label) => AsmInstruction::new_jmp(label),

            AsmInstruction::JmpCC { cond, label } => {
                AsmInstruction::new_jmp_c_c(cond, label)
            }

            AsmInstruction::Label(name) => AsmInstruction::new_label(name),
        }
    }

    fn replace_operand(&mut self, operand: AsmOperand) -> AsmOperand {
        match operand {
            AsmOperand::Pseudo(identifier) => {
                let offset = self.stack_map.entry(identifier).or_insert_with(|| {
                    let offset = self.next_offset;
                    self.next_offset -= 4;
                    offset
                });
                AsmOperand::new_stack(*offset)
            }
            other => other,
        }
    }
}
