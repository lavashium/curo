use crate::parser::rules::*;
use language::*;

pub fn parse_break(parser: &mut ParserRules, ctx: &mut ParserContext) -> Option<AstStatement> {
    let start_span = parser.parser.source_tokens.peek()?.get_span();
    parser.expect(ctx, token_keyword!(Break))?;
    parser.expect(ctx, token_punctuation!(Semicolon))?;
    let end_span = parser.parser.source_tokens.peek()?.get_span();
    let span = combine_spans!(start_span, end_span);
    Some(AstStatement::new_break(
        String::new(),
        span,
    ))
}