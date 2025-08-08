use common::*;
use super::*;

impl<'scp, 'ctx> Factory<(), TypedProgram> for LoopLabeling<'scp, 'ctx> {
    fn run(program: &mut TypedProgram, ctx: &mut AnalyzerContext<'scp, 'ctx>) -> () {
        for decl in program.declarations_mut() {
            Self::run(decl, ctx)
        }
    }
}