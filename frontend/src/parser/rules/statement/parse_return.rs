use crate::parser::rules::*;
use language::*;

pub fn parse_return(parser: &mut ParserRules) -> ParseResult<AstStatement> {
    parser.expect(token_keyword!(Return))?;
    let expression = parser.parse_expression()?;
    parser.expect(token_punctuation!(Semicolon))?;
    Some(AstStatement::new_return(
        expression,
    ))
}