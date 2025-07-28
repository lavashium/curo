use super::*;
use crate::asm::*;
use language::*;
use common::*;

impl Factory<AsmProgram, TacProgram, ()> for GeneratorCasts {
    fn run(program: &mut TacProgram, ctx: &mut ()) -> AsmProgram {
        let function_definitions = program.function_definitions_mut()
            .iter_mut()
            .map(|f| Self::run(f, ctx))
            .collect();
        AsmProgram::new(function_definitions)
    }
}