use common::*;
use super::*;

impl Factory<(), TypedFunctionDeclaration, AnalyzerContext<'_, '_>> for TypeCheck {
    fn run(fun: &mut TypedFunctionDeclaration, ctx: &mut AnalyzerContext) {
        let name = fun.identifier().clone();
        let param_count = fun.params().len();
        let fun_ty = AstType::FunType(param_count);
        let has_body = fun.body().is_some();
        let span = fun.get_span();

        let symtable = &mut ctx.ctx.symtable;
        if let Some(old_symbol) = symtable.get(&name) {
            if old_symbol.ty() != &fun_ty {
                ctx.ctx.diagnostics.push(Diagnostic::error(
                    span,
                    DiagnosticKind::Custom(format!("Redeclared function {} with a different type", name)),
                ));
            } else if old_symbol.get_defined() && has_body {
                ctx.ctx.diagnostics.push(Diagnostic::error(
                    span,
                    DiagnosticKind::Custom(format!("Defined body of function {} twice", name)),
                ));
            }
        }

        let already_defined = *symtable.get(&name).map(|s| s.defined()).unwrap_or(&false);
        let new_defined = already_defined || has_body;
        symtable.add_fun(name.clone(), fun_ty, new_defined);
        let params = fun.get_params();
        if let Some(body) = fun.body_mut() {
            for param in params {
                symtable.add_var(param.clone(), AstType::Int);
            }

            TypeCheck::run(body, ctx);
        }
    }
}