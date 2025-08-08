use common::*;
use super::*;

impl<'scp, 'ctx> Factory<(), TypedBlockItem> for TypeCheck<'scp, 'ctx> {
    fn run(item: &mut TypedBlockItem, ctx: &mut AnalyzerContext<'scp, 'ctx>) {
        match item {
            TypedBlockItem::Statement(stmt) => Self::run(stmt, ctx),
            TypedBlockItem::Declaration(decl) => Self::run(decl, ctx),
        }
    }
}