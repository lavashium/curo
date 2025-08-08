use super::*;
use language::*;
use common::*;

impl<'scp, 'ctx> Factory<Vec<TacInstruction>, TypedForInit> for GeneratorTransforms<'scp, 'ctx> {
    fn run(for_init: &mut TypedForInit, ctx: &mut TacGenContext<'scp, 'ctx>) -> Vec<TacInstruction> {
        let mut instructions = Vec::new();
        match for_init { 
            TypedForInit::InitDeclaration{decl, ..} => {
                let mut var_decl = TypedDeclaration::VarDecl(decl.clone());
                instructions.append(&mut Self::run(&mut var_decl, ctx));
            }
            TypedForInit::InitExpression{expr: Some(expr), ..} => {
                let (mut expr_instrs, _) = Self::run(expr, ctx);
                instructions.append(&mut expr_instrs);
            }
            TypedForInit::InitExpression{expr: None, ..} => {}
        }
        instructions
    }
}