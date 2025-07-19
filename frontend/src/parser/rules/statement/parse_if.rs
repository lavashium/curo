use crate::parser::rules::*;
use language::*;

pub fn parse_if(parser: &mut ParserRules, ctx: &mut ParserContext) -> Option<AstStatement> {
    let start_span = parser.parser.source_tokens.peek()?.get_span();
    parser.expect(ctx, token_keyword!(If))?;
    parser.expect(ctx, token_punctuation!(OpenParen))?;
    let condition = parser.parse_expression(ctx)?;
    parser.expect(ctx, token_punctuation!(CloseParen))?;
    let then_branch = parser.parse_statement(ctx)?;
    let else_branch = match parser.parser.source_tokens.peek()?.kind() {
        TokenKind::Keyword(KeywordKind::Else) => {
            parser.expect(ctx, token_keyword!(Else))?;
            Some(Box::new(parser.parse_statement(ctx)?))
        },
        _ => None,
    };
    
    let end_span = parser.parser.source_tokens.peek()?.get_span();
    let span = combine_spans!(start_span, end_span);
    Some(AstStatement::new_if(
        condition,
        Box::new(then_branch),
        else_branch,
        span
    ))
}
