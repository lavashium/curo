use super::*;
use crate::asm::*;

pub struct BinopLegalizer;

impl Legalizer for BinopLegalizer {
    fn legalize(instr: &AsmInstruction) -> Option<Vec<AsmInstruction>> {
        match instr {
            AsmInstruction::Binary {
                binary_operator: AsmBinaryOperator::Mult,
                src,
                dst,
            } if is_stack_operand(&dst) => {
                Some(vec![
                    AsmInstruction::Mov {
                        src: src.clone(),
                        dst: AsmOperand::Reg(AsmReg::R11),
                    },
                    AsmInstruction::Binary {
                        binary_operator: AsmBinaryOperator::Mult,
                        src: dst.clone(),
                        dst: AsmOperand::Reg(AsmReg::R11),
                    },
                    AsmInstruction::Mov {
                        src: AsmOperand::Reg(AsmReg::R11),
                        dst: dst.clone(),
                    },
                ])
            },

            AsmInstruction::Binary {
                binary_operator,
                src,
                dst,
            } if both_stack_operands(&src, &dst) => {
                Some(vec![
                    AsmInstruction::Mov {
                        src: src.clone(),
                        dst: AsmOperand::Reg(AsmReg::R10),
                    },
                    AsmInstruction::Binary {
                        binary_operator: binary_operator.clone(),
                        src: AsmOperand::Reg(AsmReg::R10),
                        dst: dst.clone(),
                    },
                ])
            },

            AsmInstruction::Idiv { 
                operand,
            } if matches!(operand, AsmOperand::Imm(_)) => {
                Some(vec![
                    AsmInstruction::Mov {
                        src: operand.clone(),
                        dst: AsmOperand::Reg(AsmReg::R10),
                    },
                    AsmInstruction::Idiv {
                        operand: AsmOperand::Reg(AsmReg::R10),
                    },
                ])
            },

            _ => None,
        }
    }
}
