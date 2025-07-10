use crate::parser::rules::*;
use language::*;

pub fn parse_do(parser: &mut ParserRules) -> ParseResult<AstStatement> {
    parser.expect(token_keyword!(Do))?;
    let body = parser.parse_statement()?;

    parser.expect(token_keyword!(While))?;
    parser.expect(token_punctuation!(OpenParen))?;
    let condition = parser.parse_expression()?;
    parser.expect(token_punctuation!(CloseParen))?;
    parser.expect(token_punctuation!(Semicolon))?;

    Some(AstStatement::new_do_while (
        condition,
        Box::new(body),
        String::new(),
    ))
}
