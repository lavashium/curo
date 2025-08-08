use crate::*;
use super::*;
use common::*;
use language::*;

impl<'scp, 'ctx> Factory<Option<AstForInit>, TokenStream> for ParserRules<'scp, 'ctx> {
    fn run(stream: &mut TokenStream, ctx: &mut ParserContext<'scp, 'ctx>) -> Option<AstForInit> {
        let start_span = stream.peek()?.get_span();
        if matches!(
            stream.peek()?.kind(),
            TokenKind::Keyword(KeywordKind::Int)
                | TokenKind::Keyword(KeywordKind::Static)
                | TokenKind::Keyword(KeywordKind::Extern)
        ) {
            match Self::run(stream, ctx) {
                Some(AstDeclaration::VarDecl(vd)) => {
                    let end_span = stream.peek()?.get_span();
                    let span = combine_spans!(start_span, end_span);
                    Some(AstForInit::new_init_declaration(vd, span))
                }
                _ => {
                    ctx.ctx.diagnostics.push(Diagnostic::error(
                        stream.peek()?.get_span(),
                        DiagnosticKind::Custom(CustomError::Message(
                            "function declaration not allowed in for loop".into(),
                        )),
                    ));
                    None
                }
            }
        } else {
            let expr = if stream.peek()?.kind() != &TokenKind::Punctuation(PunctuationKind::Semicolon) {
                Some(try_apply!(Self, _, stream, ctx))
            } else {
                None
            };
            Self::expect(stream, ctx, TokenKind::Punctuation(PunctuationKind::Semicolon))?;
            let end_span = stream.peek()?.get_span();
            let span = combine_spans!(start_span, end_span);
            Some(AstForInit::new_init_expression(
                expr,
                span
            ))
        }
    }
}
