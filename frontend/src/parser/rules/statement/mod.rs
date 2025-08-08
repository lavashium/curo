use super::*;
use common::*;
use language::*;

mod parse_return;
mod parse_if;
mod parse_break;
mod parse_continue;
mod parse_while;
mod parse_do;
mod parse_for;

impl<'scp, 'ctx> Factory<Option<AstStatement>, TokenStream> for ParserRules<'scp, 'ctx> {
    fn run(stream: &mut TokenStream, ctx: &mut ParserContext<'scp, 'ctx>) -> Option<AstStatement> {
        match stream.peek()?.kind() {
            TokenKind::Keyword(KeywordKind::Return)   => Self::parse_return  (stream, ctx),
            TokenKind::Keyword(KeywordKind::If)       => Self::parse_if      (stream, ctx),
            TokenKind::Keyword(KeywordKind::Break)    => Self::parse_break   (stream, ctx),
            TokenKind::Keyword(KeywordKind::Continue) => Self::parse_continue(stream, ctx),
            TokenKind::Keyword(KeywordKind::While)    => Self::parse_while   (stream, ctx),
            TokenKind::Keyword(KeywordKind::Do)       => Self::parse_do      (stream, ctx),
            TokenKind::Keyword(KeywordKind::For)      => Self::parse_for     (stream, ctx),
            TokenKind::Punctuation(PunctuationKind::Semicolon) => {
                Self::expect(stream, ctx, token_punctuation!(Semicolon))?;
                Some(AstStatement::new_null())
            }
            TokenKind::Punctuation(PunctuationKind::OpenBrace) => {
                let start_span = &stream.peek()?.get_span();
                let block = try_apply!(Self, _, stream, ctx);
                let end_span = &stream.peek()?.get_span();
                let span = combine_spans!(start_span, end_span);
                Some(AstStatement::new_compound(
                    block,
                    span
                ))
            }
            _ => {
                let start_span = &stream.peek()?.get_span();
                let expr = Self::parse_expression(stream, ctx)?;
                Self::expect(stream, ctx, token_punctuation!(Semicolon))?;
                let end_span = &stream.peek()?.get_span();
                let span = combine_spans!(start_span, end_span);
                Some(AstStatement::new_expression(
                    expr,
                    span
                ))
            }
        }
    }
}
