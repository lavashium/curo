use common::*;
use super::*;

impl Factory<(), TypedBlockItem, AnalyzerContext<'_, '_>> for TypeCheck {
    fn run(item: &mut TypedBlockItem, ctx: &mut AnalyzerContext) {
        match item {
            TypedBlockItem::Statement(stmt) => TypeCheck::run(stmt, ctx),
            TypedBlockItem::Declaration(decl) => TypeCheck::run(decl, ctx),
        }
    }
}