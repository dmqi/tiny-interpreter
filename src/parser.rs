use super::ast::*;
use super::lexer::Lexer;
use super::token::Token;

use std::mem;

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone)]
pub enum Precedence {

}

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    cur_token: Token,
    peek_token: Token,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Self {
        Parser {
            lexer,
            cur_token: Token::Illegal,
            peek_token: Token::Illegal,
        }
    }
}
