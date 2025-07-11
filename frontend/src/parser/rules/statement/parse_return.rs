use crate::parser::rules::*;
use language::*;

pub fn parse_return(parser: &mut ParserRules) -> ParseResult<AstStatement> {
    let start_span = parser.parser.source_tokens.peek()?.get_span();
    parser.expect(token_keyword!(Return))?;
    let expression = parser.parse_expression()?;
    parser.expect(token_punctuation!(Semicolon))?;
    let end_span = parser.parser.source_tokens.peek()?.get_span();
    let span = combine_spans!(start_span, end_span);
    Some(AstStatement::new_return(
        expression,
        span
    ))
}