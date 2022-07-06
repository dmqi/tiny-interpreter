use super::token::*;

use std::str::Chars;
use std::iter::Peekable;

pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer { input: input.chars().peekable() }
    }

    pub fn read_char(&mut self) -> Option<char> {
        self.input.next()
    }

    pub fn peek_char(&mut self) -> Option<&char> {
        self.input.peek()
    }

    pub fn read_str(&mut self, ch: char) -> String {
        let mut str = String::new();
        str.push(ch);
        while self.is_letter() {
            str.push(self.read_char().unwrap());
        }
        str
    }

    pub fn read_num(&mut self, ch: char) -> String {
        let mut str = String::new();
        str.push(ch);
        while self.is_number() {
            str.push(self.read_char().unwrap());
        }
        str
    }

    pub fn skip(&mut self) {
        while let Some(&c) = self.input.peek() {
            if !c.is_whitespace() {
                break;
            }
            self.input.next();
        }
    }

    pub fn is_letter(&mut self) -> bool {
        match self.input.peek() {
            Some(&c) => c.is_alphabetic(),
            None => false,
        }
    }

    pub fn is_number(&mut self) -> bool {
        match self.input.peek() {
            Some(&c) => c.is_numeric(),
            None => false,
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip();
        match self.read_char() {
            Some(':') => {
                self.read_char();
                Token::Assign
            }
            Some('=') => Token::Define,
            Some('+') => Token::Add,
            Some('-') => Token::Sub,
            Some('*') => Token::Mul,
            Some('/') => Token::Div,
            Some('(') => Token::LeftBracket,
            Some(')') => Token::RightBracket,
            Some(',') => Token::Comma,
            Some(ch) => {
                if ch.is_alphabetic() {
                    let str = self.read_str(ch);
                    lookup(&str)
                } else if ch.is_numeric() {
                    let str = self.read_num(ch);
                    Token::Integer(str)
                } else {
                    Token::Illegal
                }
            }
            None => Token::End,
        }
    }
}

#[cfg(test)] 
mod tests {
    use super::Token;
    use super::*;

    #[test]
    fn test_arithmatic_expression() {
        let str = "(1+2)/2";
        let mut lexer = Lexer::new(str);
        assert_eq!(lexer.next_token(), Token::LeftBracket);
        assert_eq!(lexer.next_token(), Token::Integer("1".to_string()));
        assert_eq!(lexer.next_token(), Token::Add);
        assert_eq!(lexer.next_token(), Token::Integer("2".to_string()));
        assert_eq!(lexer.next_token(), Token::RightBracket);
        assert_eq!(lexer.next_token(), Token::Div);
        assert_eq!(lexer.next_token(), Token::Integer("2".to_string()));
        assert_eq!(lexer.next_token(), Token::End);
    }

    #[test]
    fn test_variable() {
        let str = "var a = 1
        a := a + 1";
        let mut lexer = Lexer::new(str);
        assert_eq!(lexer.next_token(), Token::Var);
        assert_eq!(lexer.next_token(), Token::Ident("a".to_string()));
        assert_eq!(lexer.next_token(), Token::Define);
        assert_eq!(lexer.next_token(), Token::Integer("1".to_string()));
        assert_eq!(lexer.next_token(), Token::Ident("a".to_string()));
        assert_eq!(lexer.next_token(), Token::Assign);
        assert_eq!(lexer.next_token(), Token::Ident("a".to_string()));
        assert_eq!(lexer.next_token(), Token::Add);
        assert_eq!(lexer.next_token(), Token::Integer("1".to_string()));
        assert_eq!(lexer.next_token(), Token::End);
    }

    #[test]
    fn test_functions() {
        let str = "fun add(a, b)=a+b
        add(1, 2) + 1";
        let mut lexer = Lexer::new(str);
        assert_eq!(lexer.next_token(), Token::Function);
        assert_eq!(lexer.next_token(), Token::Ident("add".to_string()));
        assert_eq!(lexer.next_token(), Token::LeftBracket);
        assert_eq!(lexer.next_token(), Token::Ident("a".to_string()));
        assert_eq!(lexer.next_token(), Token::Comma);
        assert_eq!(lexer.next_token(), Token::Ident("b".to_string()));
        assert_eq!(lexer.next_token(), Token::RightBracket);
        assert_eq!(lexer.next_token(), Token::Define);
        assert_eq!(lexer.next_token(), Token::Ident("a".to_string()));
        assert_eq!(lexer.next_token(), Token::Add);
        assert_eq!(lexer.next_token(), Token::Ident("b".to_string()));
        assert_eq!(lexer.next_token(), Token::Ident("add".to_string()));
        assert_eq!(lexer.next_token(), Token::LeftBracket);
        assert_eq!(lexer.next_token(), Token::Integer("1".to_string()));
        assert_eq!(lexer.next_token(), Token::Comma);
        assert_eq!(lexer.next_token(), Token::Integer("2".to_string()));
        assert_eq!(lexer.next_token(), Token::RightBracket);
        assert_eq!(lexer.next_token(), Token::Add);
        assert_eq!(lexer.next_token(), Token::Integer("1".to_string()));
        assert_eq!(lexer.next_token(), Token::End);
    }
}
