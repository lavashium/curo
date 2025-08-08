use common::*;
use super::*;

impl<'scp, 'ctx> Factory<(), TypedProgram> for IdentifierResolution<'scp, 'ctx> {
    fn run(program: &mut TypedProgram, ctx: &mut AnalyzerContext<'scp, 'ctx>) -> () {
        ctx.global_scope = true;
        ctx.inside_function = false;
        
        for decl in program.declarations_mut() {
            Self::run(decl, ctx)
        }

        ctx.global_scope = false;
        ctx.inside_function = false;
    }
}