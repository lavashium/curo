use crate::parser::rules::*;
use language::*;

pub fn parse_while(parser: &mut ParserRules) -> ParseResult<AstStatement> {
    parser.expect(token_keyword!(While))?;
    parser.expect(token_punctuation!(OpenParen))?;
    let condition = parser.parse_expression()?;
    parser.expect(token_punctuation!(CloseParen))?;
    let body = parser.parse_statement()?;
    Some(AstStatement::new_while(
        condition,
        Box::new(body),
        String::new(),
    ))
}
