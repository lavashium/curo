use crate::parser::rules::*;
use language::*;

pub fn parse_while(parser: &mut ParserRules) -> ParseResult<AstStatement> {
    let start_span = parser.parser.source_tokens.peek()?.get_span();
    parser.expect(token_keyword!(While))?;
    parser.expect(token_punctuation!(OpenParen))?;
    let condition = parser.parse_expression()?;
    parser.expect(token_punctuation!(CloseParen))?;
    let body = parser.parse_statement()?;
    let end_span = parser.parser.source_tokens.peek()?.get_span();
    let span = combine_spans!(start_span, end_span);
    Some(AstStatement::new_while(
        condition,
        Box::new(body),
        String::new(),
        span
    ))
}
