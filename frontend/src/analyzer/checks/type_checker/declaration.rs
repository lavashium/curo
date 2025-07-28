use common::*;
use super::*;

impl Factory<(), TypedDeclaration, AnalyzerContext<'_, '_>> for TypeCheck {
    fn run(decl: &mut TypedDeclaration, ctx: &mut AnalyzerContext) {
        match decl {
            TypedDeclaration::FunDecl(fd) => Self::run(fd, ctx),
            TypedDeclaration::VarDecl(vd) => Self::run(vd, ctx),
        }
    }
}