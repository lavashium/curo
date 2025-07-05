use super::*;
use crate::asm::*;
use language::*;

pub trait FunctionCast {
    fn cast_function(&self, function: &TacFunction) -> AsmFunction;
}

impl<'a> FunctionCast for GeneratorCasts<'a> {
    fn cast_function(&self, function: &TacFunction) -> AsmFunction {
        let identifier = function.get_identifier();
        let mut instructions = Vec::new();
        for instruction in function.instructions() {
            let mut instruction = self.cast_instruction(instruction);
            instructions.append(&mut instruction);
        }
        AsmFunction::new(identifier, instructions)
    }
}
