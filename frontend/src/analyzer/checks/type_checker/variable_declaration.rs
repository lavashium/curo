use common::*;
use super::*;

impl Factory<(), TypedVariableDeclaration, AnalyzerContext<'_, '_>> for TypeCheck {
    fn run(vd: &mut TypedVariableDeclaration, ctx: &mut AnalyzerContext) {
        ctx.ctx.symtable.add_var(vd.identifier().clone(), AstType::Int);
        Self::run_option(vd.init_mut(), ctx);
    }
}