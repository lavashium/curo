use crate::asm::*;

pub struct AsmLegalizer {
    stack_size: i32,
}

impl AsmLegalizer {
    pub fn new(stack_size: i32) -> Self {
        Self { stack_size }
    }

    pub fn legalize(&self, program: AsmProgram) -> AsmProgram {
        let mut function = program.function_definition;

        let mut instructions = Vec::with_capacity(function.instructions.len() + 1);
        instructions.push(AsmInstruction::AllocateStack(self.stack_size));

        for instr in function.instructions {
            instructions.extend(self.fix_instruction(instr));
        }

        function.instructions = instructions;

        AsmProgram { function_definition: function }
    }

    fn fix_instruction(&self, instr: AsmInstruction) -> Vec<AsmInstruction> {
        match instr {
            AsmInstruction::Mov { src, dst } if self.both_stack_operands(&src, &dst) => {
                vec![
                    AsmInstruction::Mov {
                        src,
                        dst: AsmOperand::Reg(AsmReg::R10),
                    },
                    AsmInstruction::Mov {
                        src: AsmOperand::Reg(AsmReg::R10),
                        dst,
                    },
                ]
            }
            _ => vec![instr],
        }
    }

    fn both_stack_operands(&self, src: &AsmOperand, dst: &AsmOperand) -> bool {
        matches!(src, AsmOperand::Stack(_)) && matches!(dst, AsmOperand::Stack(_))
    }
}
