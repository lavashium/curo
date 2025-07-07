use super::*;
use language::*;

pub trait DeclarationTransform {
    fn transform_declaration(&mut self, declaration: &AstDeclaration) -> Vec<TacInstruction>;
}

impl<'a> DeclarationTransform for GeneratorTransforms<'a> {
    fn transform_declaration(&mut self, declaration: &AstDeclaration) -> Vec<TacInstruction> {
        let mut instructions = vec![];

        let var_name = declaration.name();
        let tac_var = TacVal::new_var(var_name.clone());

        if let Some(init_expr) = declaration.init() {
            let (mut expr_instrs, value) = self.transform_expression(init_expr);
            instructions.append(&mut expr_instrs);

            instructions.push(TacInstruction::Copy {
                src: value,
                dst: tac_var,
            });
        }

        instructions
    }
}
