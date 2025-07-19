use super::*;
use language::*;
use common::*;

impl<'scp, 'ctx> GeneratorTransforms<'scp, 'ctx> {
    pub fn transform_for_init(&mut self, item: &mut AstForInit) -> Vec<TacInstruction> {
        <Self as Factory<Vec<TacInstruction>, Self, AstForInit>>::run(self, item)
    }
}

impl<'scp, 'ctx> Factory<Vec<TacInstruction>, Self, AstForInit> for GeneratorTransforms<'scp, 'ctx> {
    fn run(driver: &mut Self, for_init: &mut AstForInit) -> Vec<TacInstruction> {
        let mut instructions = Vec::new();
        match for_init {
            AstForInit::InitDeclaration{decl, ..} => {
                let mut var_decl = AstDeclaration::VarDecl(decl.clone());
                instructions.append(&mut driver.transform_declaration(&mut var_decl));
            }
            AstForInit::InitExpression{expr: Some(expr), ..} => {
                let (mut expr_instrs, _) = driver.transform_expression(expr);
                instructions.append(&mut expr_instrs);
            }
            AstForInit::InitExpression{expr: None, ..} => {}
        }
        instructions
    }
}