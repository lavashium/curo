use crate::asm::*;
use language::*;
use super::GeneratorCasts;

impl<'a> GeneratorCasts<'a> {
    pub fn cast_unary(&self, instruction: &TacInstruction) -> Vec<AsmInstruction> {
        if let TacInstruction::Unary { operator, source, destination } = instruction {
            vec![
                AsmInstruction::Mov {
                    src: convert_operand(source),
                    dst: convert_operand(destination),
                },
                AsmInstruction::Unary {
                    unary_operator: convert_unary_operator(operator),
                    operand: convert_operand(destination),
                },
            ]
        } else {
            unreachable!()
        }
    }
}
