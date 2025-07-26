use common::*;
use super::*;

impl Factory<(), TypedVariableDeclaration, AnalyzerContext<'_, '_>> for TypeCheck {
    fn run(vd: &mut TypedVariableDeclaration, ctx: &mut AnalyzerContext) {
        ctx.ctx.symtable.add_var(vd.identifier().clone(), AstType::Int);
        if let Some(init) = vd.init_mut() {
            TypeCheck::run(init, ctx);
        }
    }
}