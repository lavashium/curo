use super::*;
use language::*;
use common::*;

impl<'scp, 'ctx> Factory<Vec<TacInstruction>, TypedDeclaration> for GeneratorTransforms<'scp, 'ctx> {
    fn run(declaration: &mut TypedDeclaration, ctx: &mut TacGenContext<'scp, 'ctx>) -> Vec<TacInstruction> {
        match declaration {
            TypedDeclaration::VarDecl(var_decl) => Self::run(var_decl, ctx),
            TypedDeclaration::FunDecl(_) => vec![]
        }
    }
}