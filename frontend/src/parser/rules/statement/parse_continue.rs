use crate::parser::rules::*;
use language::*;

pub fn parse_continue(parser: &mut ParserRules, ctx: &mut ParserContext) -> Option<AstStatement> {
    let start_span = parser.parser.source_tokens.peek()?.get_span();
    parser.expect(ctx, token_keyword!(Continue))?;
    parser.expect(ctx, token_punctuation!(Semicolon))?;
    let end_span = parser.parser.source_tokens.peek()?.get_span();
    let span = combine_spans!(start_span, end_span);
    Some(AstStatement::new_continue(
        String::new(),
        span
    ))
}