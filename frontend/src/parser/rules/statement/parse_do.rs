use crate::parser::rules::*;
use language::*;

pub fn parse_do(parser: &mut ParserRules, ctx: &mut ParserContext) -> Option<AstStatement> {
    let start_span = parser.parser.source_tokens.peek()?.get_span();
    parser.expect(ctx, token_keyword!(Do))?;
    let body = parser.parse_statement(ctx)?;
    parser.expect(ctx, token_keyword!(While))?;
    parser.expect(ctx, token_punctuation!(OpenParen))?;
    let condition = parser.parse_expression(ctx)?;
    parser.expect(ctx, token_punctuation!(CloseParen))?;
    parser.expect(ctx, token_punctuation!(Semicolon))?;
    let end_span = parser.parser.source_tokens.peek()?.get_span();
    let span = combine_spans!(start_span, end_span);

    Some(AstStatement::new_do_while (
        condition,
        Box::new(body),
        String::new(),
        span
    ))
}
