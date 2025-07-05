use super::GeneratorCasts;
use crate::asm::*;
use language::*;

impl<'a> GeneratorCasts<'a> {
    pub fn cast_unary(&self, instruction: &TacInstruction) -> Vec<AsmInstruction> {
        if let TacInstruction::Unary {
            operator,
            source,
            destination,
        } = instruction {
            match operator {
                TacUnaryKind::Not => {
                    vec![
                        AsmInstruction::new_cmp(
                            AsmOperand::new_imm(0),
                            convert_operand(source)
                        ),
                        AsmInstruction::new_mov(
                            AsmOperand::new_imm(0), 
                            convert_operand(destination),
                        ),
                        AsmInstruction::new_set_c_c(
                            AsmCondCode::E,
                            convert_operand(destination)
                        )
                    ]
                },
                _ => {
                    vec![
                        AsmInstruction::new_mov(
                            convert_operand(source),
                            convert_operand(destination),
                        ),
                        AsmInstruction::new_unary(
                            convert_unary_operator(operator),
                            convert_operand(destination),
                        ),
                    ]
                }
            }
        } else {
            unreachable!()
        }
    }
}
