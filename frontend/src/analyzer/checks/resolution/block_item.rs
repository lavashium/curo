use common::*;
use super::*;

impl Factory<(), TypedBlockItem, AnalyzerContext<'_, '_>> for IdentifierResolution {
    fn run(item: &mut TypedBlockItem, ctx: &mut AnalyzerContext) {
        match item {
            TypedBlockItem::Statement(stmt)   => Self::run(stmt, ctx),
            TypedBlockItem::Declaration(decl) => match decl {
                TypedDeclaration::VarDecl(v)  => Self::run(v, ctx),
                TypedDeclaration::FunDecl(f)  => Self::run(f, ctx),
            },
        }
    }
}