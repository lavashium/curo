use super::*;
use crate::asm::*;

pub struct MovLegalizer;

impl Legalizer for MovLegalizer {
    fn legalize(instr: &AsmInstruction) -> Option<Vec<AsmInstruction>> {
        match instr {
            AsmInstruction::Mov {
                src,
                dst
            } if both_stack_operands(&src, &dst) => {
                Some(vec![
                    AsmInstruction::Mov {
                        src: src.clone(),
                        dst: AsmOperand::Reg(AsmReg::R10),
                    },
                    AsmInstruction::Mov {
                        src: AsmOperand::Reg(AsmReg::R10),
                        dst: dst.clone(),
                    },
                ])
            },

            _ => None,
        }
    }
}
