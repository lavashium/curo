use super::*;
use language::*;
use common::*;

impl<'scp, 'ctx> GeneratorTransforms<'scp, 'ctx> {
    pub fn transform_for_init(&mut self, item: &mut TypedForInit) -> Vec<TacInstruction> {
        <Self as Factory<Vec<TacInstruction>, Self, TypedForInit>>::run(self, item)
    }
}

impl<'scp, 'ctx> Factory<Vec<TacInstruction>, Self, TypedForInit> for GeneratorTransforms<'scp, 'ctx> {
    fn run(driver: &mut Self, for_init: &mut TypedForInit) -> Vec<TacInstruction> {
        let mut instructions = Vec::new();
        match for_init {
            TypedForInit::InitDeclaration{decl, ..} => {
                let mut var_decl = TypedDeclaration::VarDecl(decl.clone());
                instructions.append(&mut driver.transform_declaration(&mut var_decl));
            }
            TypedForInit::InitExpression{expr: Some(expr), ..} => {
                let (mut expr_instrs, _) = driver.transform_expression(expr);
                instructions.append(&mut expr_instrs);
            }
            TypedForInit::InitExpression{expr: None, ..} => {}
        }
        instructions
    }
}