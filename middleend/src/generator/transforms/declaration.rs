use super::*;
use language::*;
use common::*;

impl<'scp, 'ctx> GeneratorTransforms<'scp, 'ctx> {
    pub fn transform_declaration(&mut self, block: &mut TypedDeclaration) -> Vec<TacInstruction> {
        <Self as Factory<Vec<TacInstruction>, Self, TypedDeclaration>>::run(self, block)
    }
}

impl<'scp, 'ctx> Factory<Vec<TacInstruction>, Self, TypedDeclaration> for GeneratorTransforms<'scp, 'ctx> {
    fn run(driver: &mut Self, declaration: &mut TypedDeclaration) -> Vec<TacInstruction> {
        match declaration {
            TypedDeclaration::VarDecl(var_decl) => driver.transform_variable_declaration(var_decl),
            TypedDeclaration::FunDecl(_) => vec![]
        }
    }
}