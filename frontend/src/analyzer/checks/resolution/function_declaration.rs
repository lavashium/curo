use language::*;
use super::*;

pub fn resolve_function_declaration(
    func: &mut AstFunctionDeclaration,
    ctx: &mut SemanticContext,
    map: &mut IdentifierMap,
) {
    if is_illegal_nested_definition(func, ctx) {
        report_nested_function_error(ctx);
        return;
    }

    if !declare_function_name(func, ctx, map) {
        return;
    }

    {
        let mut scratch = IdentifierMap::new();
        for param in func.params() {
            let _ = declare_identifier(param, false, Span::default(), ctx, &mut scratch);
        }
    }

    if func.body().is_none() {
        return;
    }

    resolve_function_definition(func, ctx, map);
}

fn is_illegal_nested_definition(
    func: &AstFunctionDeclaration,
    ctx: &SemanticContext,
) -> bool {
    ctx.inside_function && func.body().is_some()
}

fn report_nested_function_error(ctx: &mut SemanticContext) {
    push_error(
        ctx,
        Span::default(),
        DiagnosticKind::NestedFunctionDefinition,
    );
}

fn declare_function_name(
    func: &AstFunctionDeclaration,
    ctx: &mut SemanticContext,
    map: &mut IdentifierMap,
) -> bool {
    declare_identifier(
        func.identifier(),
        true,
        Span::default(),
        ctx,
        map,
    )
}

fn resolve_function_definition(
    func: &mut AstFunctionDeclaration,
    ctx: &mut SemanticContext,
    outer_map: &mut IdentifierMap,
) {
    ctx.inside_function = true;
    ctx.push_scope();

    let mut fn_scope = copy_identifier_map(outer_map);
    declare_parameters(func, ctx, &mut fn_scope);

    if let Some(body) = func.body_mut() {
        for item in body.block_items_mut() {
            resolve_block_item(item, ctx, &mut fn_scope);
        }
    }

    ctx.pop_scope();
    ctx.inside_function = false;
}

fn declare_parameters(
    func: &AstFunctionDeclaration,
    ctx: &mut SemanticContext,
    fn_scope: &mut IdentifierMap,
) {
    for param in func.params() {
        let _ = declare_identifier(
            param,
            false,
            Span::default(),
            ctx,
            fn_scope,
        );
    }
}
