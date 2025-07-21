use common::*;
use super::*;

impl IdentifierResolution {
    pub fn resolve_block_item(item: &mut TypedBlockItem, ctx: &mut AnalyzerContext) {
        <Self as Factory<(), TypedBlockItem, AnalyzerContext<'_, '_>>>::run(item, ctx)
    }
}

impl Factory<(), TypedBlockItem, AnalyzerContext<'_, '_>> for IdentifierResolution {
    fn run(item: &mut TypedBlockItem, ctx: &mut AnalyzerContext) {
        match item {
            TypedBlockItem::Statement(stmt)   => Self::resolve_statement(stmt, ctx),
            TypedBlockItem::Declaration(decl) => match decl {
                TypedDeclaration::VarDecl(v)  => Self::resolve_variable_declaration(v, ctx),
                TypedDeclaration::FunDecl(f)  => Self::resolve_function_declaration(f, ctx),
            },
        }
    }
}