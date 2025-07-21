use super::*;
use language::*;
use common::*;

impl<'scp, 'ctx> GeneratorTransforms<'scp, 'ctx> {
    pub fn transform_variable_declaration(&mut self, declaration: &mut TypedVariableDeclaration) -> Vec<TacInstruction> {
        <Self as Factory<Vec<TacInstruction>, Self, TypedVariableDeclaration>>::run(self, declaration)
    }
}

impl<'scp, 'ctx> Factory<Vec<TacInstruction>, Self, TypedVariableDeclaration> for GeneratorTransforms<'scp, 'ctx> {
    fn run(driver: &mut Self, declaration: &mut TypedVariableDeclaration) -> Vec<TacInstruction> {
        let mut instructions = vec![];

        let var_name = declaration.identifier().clone();
        let tac_var = TacVal::new_var(var_name);

        if let Some(init_expr) = declaration.init_mut() {
            let (mut expr_instrs, value) = driver.transform_expression(init_expr);
            instructions.append(&mut expr_instrs);

            instructions.push(TacInstruction::new_copy(value, tac_var));
        }

        instructions
    }
}