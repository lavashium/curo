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
            } if is_stack_operand(dst) => Some(vec![
                AsmInstruction::new_mov(src.clone(), AsmOperand::new_reg(AsmReg::R11)),
                AsmInstruction::new_binary(
                    AsmBinaryOperator::Mult,
                    dst.clone(),
                    AsmOperand::new_reg(AsmReg::R11),
                ),
                AsmInstruction::new_mov(AsmOperand::new_reg(AsmReg::R11), dst.clone()),
            ]),

            AsmInstruction::Binary {
                binary_operator,
                src,
                dst,
            } if both_stack_operands(src, dst) => Some(vec![
                AsmInstruction::new_mov(src.clone(), AsmOperand::new_reg(AsmReg::R10)),
                AsmInstruction::new_binary(
                    binary_operator.clone(),
                    AsmOperand::new_reg(AsmReg::R10),
                    dst.clone(),
                ),
            ]),

            AsmInstruction::Idiv { operand } if matches!(operand, AsmOperand::Imm(_)) => {
                Some(vec![
                    AsmInstruction::new_mov(operand.clone(), AsmOperand::new_reg(AsmReg::R10)),
                    AsmInstruction::new_idiv(AsmOperand::new_reg(AsmReg::R10)),
                ])
            }

            _ => None,
        }
    }
}
