use common::*;
use super::*;

impl Factory<(), TypedDeclaration, AnalyzerContext<'_, '_>> for TypeCheck {
    fn run(decl: &mut TypedDeclaration, ctx: &mut AnalyzerContext) {
        match decl {
            TypedDeclaration::FunDecl(fd) => TypeCheck::run(fd, ctx),
            TypedDeclaration::VarDecl(vd) => TypeCheck::run(vd, ctx),
        }
    }
}