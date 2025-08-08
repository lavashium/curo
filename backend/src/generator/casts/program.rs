use super::*;
use crate::asm::*;
use language::*;
use common::*;

impl<'scp, 'ctx> Factory<AsmProgram, TacProgram> for GeneratorCasts<'scp, 'ctx> {
    fn run(program: &mut TacProgram, ctx: &mut GeneratorContext<'scp, 'ctx>) -> AsmProgram {
        let function_definitions = program.top_level_mut()
            .iter_mut()
            .map(|f| Self::run(f, ctx))
            .collect();
        AsmProgram::new(function_definitions)
    }
}