use super::*;
use language::*;
use common::*;

impl Factory<Vec<TacInstruction>, TypedDeclaration, TacGenContext<'_, '_>> for GeneratorTransforms {
    fn run(declaration: &mut TypedDeclaration, ctx: &mut TacGenContext) -> Vec<TacInstruction> {
        match declaration {
            TypedDeclaration::VarDecl(var_decl) => Self::run(var_decl, ctx),
            TypedDeclaration::FunDecl(_) => vec![]
        }
    }
}