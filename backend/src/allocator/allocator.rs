use crate::asm::*;

use std::collections::HashMap;

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
        let function = self.visit_function(program.function_definition);
        let final_offset = self.next_offset;
        (
            AsmProgram {
                function_definition: function,
            },
            -final_offset,
        )
    }

    fn visit_function(&mut self, function: AsmFunction) -> AsmFunction {
        let instructions = function
            .instructions
            .into_iter()
            .map(|instr| self.visit_instruction(instr))
            .collect();

        AsmFunction {
            identifier: function.identifier,
            instructions,
        }
    }

    fn visit_instruction(&mut self, instr: AsmInstruction) -> AsmInstruction {
        use AsmInstruction::*;
        match instr {
            Mov { src, dst } => Mov {
                src: self.replace_operand(src),
                dst: self.replace_operand(dst),
            },
            Unary {
                unary_operator,
                operand,
            } => Unary {
                unary_operator,
                operand: self.replace_operand(operand),
            },
            Ret => Ret,
            AllocateStack(_) => AllocateStack(0),
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
                AsmOperand::Stack(*offset)
            }
            other => other,
        }
    }
}
