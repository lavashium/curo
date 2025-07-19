use super::*;
use language::*;
use common::*;

impl<'scp, 'ctx> GeneratorTransforms<'scp, 'ctx> {
    pub fn transform_declaration(&mut self, block: &mut AstDeclaration) -> Vec<TacInstruction> {
        <Self as Factory<Vec<TacInstruction>, Self, AstDeclaration>>::run(self, block)
    }
}

impl<'scp, 'ctx> Factory<Vec<TacInstruction>, Self, AstDeclaration> for GeneratorTransforms<'scp, 'ctx> {
    fn run(driver: &mut Self, declaration: &mut AstDeclaration) -> Vec<TacInstruction> {
        match declaration {
            AstDeclaration::VarDecl(var_decl) => driver.transform_variable_declaration(var_decl),
            AstDeclaration::FunDecl(_) => vec![]
        }
    }
}