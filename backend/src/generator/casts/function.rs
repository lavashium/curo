use crate::asm::*;
use language::*;
use super::*;

pub trait FunctionCast {
    fn cast_function(&self, function: &TacFunction) -> AsmFunction;
}

impl<'a> FunctionCast for GeneratorCasts<'a> {
    fn cast_function(&self, function: &TacFunction) -> AsmFunction {
        let identifier = function.identifier.clone();
        let mut instructions = Vec::new();
        for instruction in function.instructions.clone() {
            let mut instruction = self.cast_instruction(&instruction);
            instructions.append(&mut instruction);
        }
        AsmFunction {
            identifier,
            instructions,
        }
    }
}