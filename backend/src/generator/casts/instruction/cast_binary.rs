use super::GeneratorCasts;
use crate::asm::*;
use language::*;

impl<'scp, 'ctx> GeneratorCasts<'scp, 'ctx> {
    pub fn cast_binary(instruction: &TacInstruction) -> Vec<AsmInstruction> {
        if let TacInstruction::Binary { operator, source1, source2, destination } = instruction {
            let src1 = convert_operand(source1);
            let src2 = convert_operand(source2);
            let dst  = convert_operand(destination);
            match operator {
                TacBinaryKind::Divide => vec![
                    AsmInstruction::new_mov(src1.clone(), AsmOperand::Reg(AsmReg::AX)),
                    AsmInstruction::new_cdq(),
                    AsmInstruction::new_idiv(src2.clone()),
                    AsmInstruction::new_mov(AsmOperand::Reg(AsmReg::AX), dst.clone()),
                ],
                TacBinaryKind::Remainder => vec![
                    AsmInstruction::new_mov(src1.clone(), AsmOperand::Reg(AsmReg::AX)),
                    AsmInstruction::new_cdq(),
                    AsmInstruction::new_idiv(src2.clone()),
                    AsmInstruction::new_mov(AsmOperand::Reg(AsmReg::DX), dst.clone()),
                ],
                TacBinaryKind::Equal
                | TacBinaryKind::NotEqual
                | TacBinaryKind::LessThan
                | TacBinaryKind::LessOrEqual
                | TacBinaryKind::GreaterThan
                | TacBinaryKind::GreaterOrEqual => vec![
                    AsmInstruction::new_cmp(src2.clone(), src1.clone()),
                    AsmInstruction::new_mov(AsmOperand::Imm(0), dst.clone()),
                    AsmInstruction::new_set_c_c(convert_to_cond_code(operator), dst.clone()),
                ],
                _ => {
                    let tmp = AsmOperand::Reg(AsmReg::R10);
                    vec![
                        AsmInstruction::new_mov(src1, tmp.clone()),
                        AsmInstruction::new_binary(convert_binary_operator(operator), src2, tmp.clone()),
                        AsmInstruction::new_mov(tmp, dst),
                    ]
                }
            }
        } else {
            unreachable!();
        }
    }

}
