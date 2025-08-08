use super::*;
use language::*;
use common::*;

impl<'scp, 'ctx> Factory<TacTopLevel, TypedFunctionDeclaration> for GeneratorTransforms<'scp, 'ctx> {
    fn run(function: &mut TypedFunctionDeclaration, ctx: &mut TacGenContext<'scp, 'ctx>) -> TacTopLevel {
        let args = function.get_params();

        let global = match ctx.ctx.symtable.get(function.identifier()) {
            Some(Symbol { attrs: IdentifierAttrs::FunAttr { global, .. }, .. }) => *global,
            _ => false,
        };

        let mut instructions = if let Some(block) = function.body_mut() {
            GeneratorTransforms::run(block, ctx)
        } else {
            vec![]
        };

        if !matches!(instructions.last(), Some(TacInstruction::Return { .. })) {
            instructions.push(TacInstruction::Return {
                val: TacVal::Constant("0".to_string()),
            });
        }

        TacTopLevel::Function {
            identifier: function.get_identifier(),
            global,
            params: args.clone(),
            instructions,
        }
    }
}