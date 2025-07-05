use crate::token::{Token, TokenKind};
use accessors::accessors;
use std::rc::Rc;

#[accessors]
#[derive(Debug, Clone)]
pub struct TokenStream {
    tokens: Rc<[Token]>,
    pointer: usize,
}

impl TokenStream {
    pub fn new(tokens: Rc<[Token]>) -> Self {
        Self { tokens, pointer: 0 }
    }

    pub fn consume(&mut self) -> Option<Token> {
        if self.pointer < self.tokens.len() {
            let tok = self.tokens[self.pointer].clone();
            self.pointer += 1;
            Some(tok)
        } else {
            None
        }
    }

    pub fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pointer)
    }

    pub fn consume_expected(&mut self, expected: &TokenKind) -> Option<Token> {
        match self.peek() {
            Some(actual) if actual.kind() == expected => self.consume(),
            _ => None,
        }
    }

    pub fn any_tokens(&self) -> bool {
        self.pointer < self.tokens.len()
    }

    pub fn is_empty(&self) -> bool {
        !self.any_tokens()
    }

    pub fn iter(&self) -> impl Iterator<Item = &Token> {
        self.tokens[self.pointer..].iter()
    }
}

impl Iterator for TokenStream {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.consume()
    }
}
