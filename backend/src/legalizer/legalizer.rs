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

        let mut instructions = Vec::with_capacity(function.instructions.len() + 3);
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

            AsmInstruction::Idiv { operand } if matches!(operand, AsmOperand::Imm(_)) => {
                vec![
                    AsmInstruction::Mov {
                        src: operand,
                        dst: AsmOperand::Reg(AsmReg::R10),
                    },
                    AsmInstruction::Idiv {
                        operand: AsmOperand::Reg(AsmReg::R10),
                    },
                ]
            }

            AsmInstruction::Binary { binary_operator: AsmBinaryOperator::Mult, operand1, operand2 }
                if self.is_stack_operand(&operand2) =>
            {   
                vec![
                    AsmInstruction::Mov {
                        src: operand1.clone(),
                        dst: AsmOperand::Reg(AsmReg::R11),
                    },
                    AsmInstruction::Binary {
                        binary_operator: AsmBinaryOperator::Mult,
                        operand1: operand2.clone(),
                        operand2: AsmOperand::Reg(AsmReg::R11),
                    },
                    AsmInstruction::Mov {
                        src: AsmOperand::Reg(AsmReg::R11),
                        dst: operand2,
                    },
                ]
            }

            AsmInstruction::Binary { binary_operator, operand1, operand2 }
                if self.both_stack_operands(&operand1, &operand2) =>
            {
                vec![
                    AsmInstruction::Mov {
                        src: operand1,
                        dst: AsmOperand::Reg(AsmReg::R10),
                    },
                    AsmInstruction::Binary {
                        binary_operator,
                        operand1: AsmOperand::Reg(AsmReg::R10),
                        operand2,
                    },
                ]
            }

            _ => vec![instr],
        }
    }

    fn both_stack_operands(&self, src: &AsmOperand, dst: &AsmOperand) -> bool {
        self.is_stack_operand(src) && self.is_stack_operand(dst)
    }

    fn is_stack_operand(&self, op: &AsmOperand) -> bool {
        matches!(op, AsmOperand::Stack(_))
    }
}
