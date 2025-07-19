use crate::parser::rules::*;
use language::*;

pub fn parse_for(parser: &mut ParserRules, ctx: &mut ParserContext) -> Option<AstStatement> {
    let start_span = parser.parser.source_tokens.peek()?.get_span();
    parser.expect(ctx, token_keyword!(For))?;
    parser.expect(ctx, token_punctuation!(OpenParen))?;
    let for_init = parser.parse_for_init(ctx)?;

    let condition = match parser.parser.source_tokens.peek()?.kind() {
        TokenKind::Punctuation(PunctuationKind::Semicolon) => {
            parser.expect(ctx, token_punctuation!(Semicolon))?;
            None
        }
        _ => {
            let cond = parser.parse_expression(ctx)?;
            parser.expect(ctx, token_punctuation!(Semicolon))?;
            Some(cond)
        }
    };

    let post = match parser.parser.source_tokens.peek()?.kind() {
        TokenKind::Punctuation(PunctuationKind::CloseParen) => {
            parser.expect(ctx, token_punctuation!(CloseParen))?;
            None
        }
        _ => {
            let post_expr = parser.parse_expression(ctx)?;
            parser.expect(ctx, token_punctuation!(CloseParen))?;
            Some(post_expr)
        }
    };

    let body = parser.parse_statement(ctx)?;
    let end_span = parser.parser.source_tokens.peek()?.get_span();
    let span = combine_spans!(start_span, end_span);

    Some(AstStatement::new_for(
        for_init,
        condition,
        post,
        Box::new(body),
        String::new(),
        span
    ))
}
