use super::*;
use language::*;
use common::*;

impl<'scp, 'ctx> Factory<TacProgram, TypedProgram> for GeneratorTransforms<'scp, 'ctx> {
    fn run(program: &mut TypedProgram, ctx: &mut TacGenContext<'scp, 'ctx>) -> TacProgram {
        let mut top_levels = vec![];

        for decl in program.declarations_mut() {
            if let TypedDeclaration::FunDecl(func_decl) = decl {
                if func_decl.body().is_some() {
                    top_levels.push(GeneratorTransforms::run(func_decl, ctx));
                }
            }
        }

        for (name, symbol) in ctx.ctx.symtable.bindings() {
            if let IdentifierAttrs::StaticAttr { init, global } = &symbol.attrs {
                let initializer = match init {
                    InitialValue::Initial(value) => value.clone(),
                    InitialValue::Tentative => "0".to_string(),
                    InitialValue::NoInitializer => { continue }
                };

                top_levels.push(TacTopLevel::StaticVariable {
                    identifier: name.clone(),
                    global: *global,
                    init: initializer,
                });
            }
        }

        TacProgram::new(top_levels)
    }
}
