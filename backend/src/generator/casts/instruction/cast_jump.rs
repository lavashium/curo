use super::GeneratorCasts;
use crate::asm::*;
use language::*;

impl GeneratorCasts {
    pub fn cast_jump(instr: &TacInstruction) -> Vec<AsmInstruction> {
        match instr {
            TacInstruction::Jump { target } => {
                vec![
                    AsmInstruction::new_jmp(
                        target.to_string()
                    ),
                ]
            }

            TacInstruction::JumpIfZero { condition, target } => {
                vec![
                    AsmInstruction::new_cmp(
                        AsmOperand::Imm(0),
                        convert_operand(condition)
                    ),
                    AsmInstruction::new_jmp_c_c(
                        AsmCondCode::E,
                        target.to_string()
                    ),
                ]
            }

            TacInstruction::JumpIfNotZero { condition, target } => {
                vec![
                    AsmInstruction::new_cmp(
                        AsmOperand::Imm(0),
                        convert_operand(condition)
                    ),
                    AsmInstruction::new_jmp_c_c(
                        AsmCondCode::NE,
                        target.to_string()
                    ),
                ]
            },

            _ => unreachable!()
        }
    }
}