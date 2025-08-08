use common::*;
use super::*;
use crate::asm::*;
use crate::*;

impl<'scp, 'ctx> Factory<Option<Vec<AsmInstruction>>, AsmInstruction> for LegalizerLegalizations<'scp, 'ctx> {
    fn run(instr: &mut AsmInstruction, _ctx: &mut LegalizerContext<'scp, 'ctx>) -> Option<Vec<AsmInstruction>> {
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

            AsmInstruction::Cmp { operand1, operand2 }
                if is_stack_operand(operand1) && is_stack_operand(operand2) =>
            {
                Some(vec![
                    AsmInstruction::new_mov(operand1.clone(), AsmOperand::new_reg(AsmReg::R10)),
                    AsmInstruction::new_cmp(AsmOperand::new_reg(AsmReg::R10), operand2.clone()),
                ])
            }

            AsmInstruction::Cmp { operand1, operand2 }
                if matches!(operand2, AsmOperand::Imm(_)) =>
            {
                Some(vec![
                    AsmInstruction::new_mov(operand2.clone(), AsmOperand::new_reg(AsmReg::R11)),
                    AsmInstruction::new_cmp(operand1.clone(), AsmOperand::new_reg(AsmReg::R11)),
                ])
            }

            AsmInstruction::Mov { src, dst } if both_stack_operands(src, dst) => Some(vec![
                AsmInstruction::new_mov(src.clone(), AsmOperand::new_reg(AsmReg::R10)),
                AsmInstruction::new_mov(AsmOperand::new_reg(AsmReg::R10), dst.clone()),
            ]),

            _ => None,
        }
    }
}

pub fn both_stack_operands(src: &AsmOperand, dst: &AsmOperand) -> bool {
    is_stack_operand(src) && is_stack_operand(dst)
}

pub fn is_stack_operand(op: &AsmOperand) -> bool {
    matches!(op, AsmOperand::Stack(_) | AsmOperand::Data(_))
}