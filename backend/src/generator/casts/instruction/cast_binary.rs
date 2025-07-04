use crate::asm::*;
use language::*;
use super::GeneratorCasts;

impl<'a> GeneratorCasts<'a> {
    pub fn cast_binary(&self, instruction: &TacInstruction) -> Vec<AsmInstruction> {
        if let TacInstruction::Binary { operator, source1, source2, destination } = instruction {
            match operator {
                BinaryKind::Divide => vec![
                    AsmInstruction::Mov {
                        src: convert_operand(source1),
                        dst: AsmOperand::Reg(AsmReg::AX),
                    },
                    AsmInstruction::Cdq,
                    AsmInstruction::Idiv {
                        operand: convert_operand(source2),
                    },
                    AsmInstruction::Mov {
                        src: AsmOperand::Reg(AsmReg::AX),
                        dst: convert_operand(destination),
                    },
                ],
                BinaryKind::Remainder => vec![
                    AsmInstruction::Mov {
                        src: convert_operand(source1),
                        dst: AsmOperand::Reg(AsmReg::AX),
                    },
                    AsmInstruction::Cdq,
                    AsmInstruction::Idiv {
                        operand: convert_operand(source2),
                    },
                    AsmInstruction::Mov {
                        src: AsmOperand::Reg(AsmReg::DX),
                        dst: convert_operand(destination),
                    },
                ],
                _ => vec![
                    AsmInstruction::Mov {
                        src: convert_operand(source1),
                        dst: convert_operand(destination),
                    },
                    AsmInstruction::Binary {
                        binary_operator: convert_binary_operator(operator),
                        src: convert_operand(source2),
                        dst: convert_operand(destination),
                    },
                ],
            }
        } else {
            unreachable!()
        }
    }
}
