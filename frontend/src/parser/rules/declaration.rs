use crate::*;
use super::*;
use common::*;
use language::*;

impl<'scp, 'ctx> Factory<Option<AstDeclaration>, TokenStream> for ParserRules<'scp, 'ctx> {
    fn run(stream: &mut TokenStream, ctx: &mut ParserContext<'scp, 'ctx>) -> Option<AstDeclaration> {
        let start_span = stream.peek()?.get_span();

        let mut saw_int = false;
        let mut storage_class: Option<AstStorageClass> = None;

        while let Some(token) = stream.peek() {
            match token.kind() {
                TokenKind::Keyword(KeywordKind::Int) => {
                    if saw_int {
                        ctx.ctx.diagnostics.push(
                            Diagnostic::error(
                                token.get_span(),
                                DiagnosticKind::Custom(CustomError::Message("duplicate type specifier".into())),
                            ),
                        );
                        return None;
                    }
                    saw_int = true;
                    stream.consume()?;
                }
                TokenKind::Keyword(KeywordKind::Static) => {
                    if storage_class.is_some() {
                        ctx.ctx.diagnostics.push(
                            Diagnostic::error(
                                token.get_span(),
                                DiagnosticKind::Semantic(SemanticError::ConflictingStorageSpecifiers)
                            ),
                        );
                        return None;
                    }
                    storage_class = Some(AstStorageClass::Static);
                    stream.consume()?;
                }
                TokenKind::Keyword(KeywordKind::Extern) => {
                    if storage_class.is_some() {
                        ctx.ctx.diagnostics.push(
                            Diagnostic::error(
                                token.get_span(),
                                DiagnosticKind::Semantic(SemanticError::ConflictingStorageSpecifiers)
                            ),
                        );
                        return None;
                    }
                    storage_class = Some(AstStorageClass::Extern);
                    stream.consume()?;
                }
                _ => break,
            }
        }

        if !saw_int {
            return None;
        }

        let name = Self::unwrap_identifier(stream, ctx)?;

        if stream.peek()?.kind() == &TokenKind::Punctuation(PunctuationKind::OpenParen) {
            stream.consume()?;
            let params = try_apply!(Self, _, stream, ctx);
            Self::expect(stream, ctx, TokenKind::Punctuation(PunctuationKind::CloseParen))?;

            let body = if stream.peek()?.kind() == &TokenKind::Punctuation(PunctuationKind::Semicolon) {
                stream.consume()?;
                None
            } else if stream.peek()?.kind() == &TokenKind::Punctuation(PunctuationKind::OpenBrace) {
                Some(try_apply!(Self, _, stream, ctx))
            } else {
                return None;
            };

            let end_span = stream.peek()?.get_span();
            let span = combine_spans!(start_span, end_span);

            Some(AstDeclaration::FunDecl(AstFunctionDeclaration::new(
                name,
                params,
                body,
                storage_class,
                span,
            )))
        } else {
            let init = if stream.peek()?.kind() == &TokenKind::Operator(OperatorKind::Equal) {
                stream.consume()?;
                Some(try_apply!(Self, _, stream, ctx))
            } else {
                None
            };

            Self::expect(stream, ctx, TokenKind::Punctuation(PunctuationKind::Semicolon))?;
            let end_span = stream.peek()?.get_span();
            let span = combine_spans!(start_span, end_span);

            Some(AstDeclaration::VarDecl(AstVariableDeclaration::new(
                name,
                init,
                storage_class,
                span,
            )))
        }
    }
}