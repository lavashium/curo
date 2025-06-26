use crate::error::manager::DiagnosticsManager;
use crate::token::*;
use crate::ast::*;
use super::rules;

pub struct Parser {
    pub source_tokens: TokenStream
}

impl Parser {
    pub fn new(source_tokens: TokenStream) -> Self {
        Parser { 
            source_tokens
        }
    }

    pub fn parse(&mut self, diagnostics: &mut DiagnosticsManager) -> Option<Program> {
        let mut rules = rules::ParserRules::new(self, diagnostics);
        rules.parse_program()
    }
}