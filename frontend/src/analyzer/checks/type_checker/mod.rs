mod program;
mod function_declaration;
mod variable_declaration;
mod block;
mod block_item;
mod declaration;
mod statement;
mod expression;

use common::*;
use language::*;
use crate::*;

pub struct TypeCheckCheck;

impl Factory<(), TypedProgram, AnalyzerContext<'_, '_>> for TypeCheckCheck {
    fn run(program: &mut TypedProgram, ctx: &mut AnalyzerContext) {
        TypeCheck::run(program, ctx);
    }
}

pub struct TypeCheck;

pub(crate) fn report_error(ctx: &mut AnalyzerContext, span: Span, message: String) {
    let diag = Diagnostic::error(span, DiagnosticKind::Custom(message));
    ctx.ctx.diagnostics_mut().push(diag);
}
