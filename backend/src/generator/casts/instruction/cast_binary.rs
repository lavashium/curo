use super::GeneratorCasts;
use crate::asm::*;
use language::*;

impl<'a> GeneratorCasts<'a> {
    pub fn cast_binary(&self, instruction: &TacInstruction) -> Vec<AsmInstruction> {
        if let TacInstruction::Binary {
            operator,
            source1,
            source2,
            destination,
        } = instruction {
            match operator {
                TacBinaryKind::Divide => vec![
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
                TacBinaryKind::Remainder => vec![
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
                |TacBinaryKind::Equal
                | TacBinaryKind::NotEqual
                | TacBinaryKind::LessThan
                | TacBinaryKind::LessOrEqual
                | TacBinaryKind::GreaterThan
                | TacBinaryKind::GreaterOrEqual => vec![
                    AsmInstruction::Cmp {
                        operand1: convert_operand(source2),
                        operand2: convert_operand(source1),
                    },
                    AsmInstruction::Mov {
                        src: AsmOperand::Imm(0),
                        dst: convert_operand(destination),
                    },
                    AsmInstruction::SetCC {
                        cond: convert_to_cond_code(operator),
                        operand: convert_operand(destination),
                    },
                ],
                _ => {
                    let tmp = AsmOperand::Reg(AsmReg::R10);
                    vec![
                        AsmInstruction::Mov {
                            src: convert_operand(source1),
                            dst: tmp.clone(),
                        },
                        AsmInstruction::Binary {
                            binary_operator: convert_binary_operator(operator),
                            src: convert_operand(source2),
                            dst: tmp.clone(),
                        },
                        AsmInstruction::Mov {
                            src: tmp,
                            dst: convert_operand(destination),
                        },
                    ]
                }
            }
        } else {
            unreachable!()
        }
    }
}
