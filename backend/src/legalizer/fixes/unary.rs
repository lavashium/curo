use super::*;
use crate::asm::*;

pub struct UnaryLegalizer;

impl Legalizer for UnaryLegalizer {
    fn legalize(_instr: &AsmInstruction) -> Option<Vec<AsmInstruction>> {
        None
    }
}
