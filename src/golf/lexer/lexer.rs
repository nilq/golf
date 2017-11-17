use super::Tokenizer;
use super::matcher::*;
use super::token::{Token, TokenType};

use std::str::Chars;
use std::rc::Rc;

pub fn lexer(data: &mut Chars) -> Lexer {
    let tokenizer = Tokenizer::new(data);
    let mut lexer = Lexer::new(tokenizer);

    let eol   = vec!["\n"].iter().map(|&x| x.to_string()).collect();

    let symbols = vec![
        "(",
        ")",
        "[",
        "]",
        ",",
        ":",
        ";",
        "{",
        "}",
        "!",
        "|",
        "=",
        "!",
    ].iter().map(|&x| x.to_string()).collect();

    let operators = vec![
        "++",
        "+",
        "-",
        "*",
        "/",
        "/",
        "%",
        "^",
        ">=",
        "<=",
        "==",
        "~=",
        ".",
        "<|",
        "|>",
        ">",
        "<",
    ].iter().map(|&x| x.to_string()).collect();

    let indent = vec![
        "  ", "\t",
    ].iter().map(|&x| x.to_string()).collect();

    let boolean = vec![
        "true",
        "false",
    ].iter().map(|&x| x.to_string()).collect();

    let matcher_eol            = ConstantMatcher::new(TokenType::EOL, eol);
    let matcher_indent         = ConstantMatcher::new(TokenType::Indent, indent);
    let matcher_operator       = ConstantMatcher::new(TokenType::Operator, operators);
    let matcher_symbol         = ConstantMatcher::new(TokenType::Symbol, symbols);
    let matcher_boolean        = KeyMatcher::new(TokenType::BoolLiteral, boolean);
    let matcher_whitespace     = WhitespaceMatcher {};
    let matcher_int_literal    = IntLiteralMatcher {};
    let matcher_float_literal  = FloatLiteralMatcher {};
    let matcher_identifier     = IdentifierMatcher {};
    let matcher_string_literal = StringLiteralMatcher {};

    lexer.matchers_mut().push(Rc::new(matcher_eol));
    lexer.matchers_mut().push(Rc::new(matcher_indent));
    lexer.matchers_mut().push(Rc::new(matcher_whitespace));
    lexer.matchers_mut().push(Rc::new(matcher_operator));
    lexer.matchers_mut().push(Rc::new(matcher_symbol));
    lexer.matchers_mut().push(Rc::new(matcher_float_literal));
    lexer.matchers_mut().push(Rc::new(matcher_int_literal));
    lexer.matchers_mut().push(Rc::new(matcher_string_literal));
    lexer.matchers_mut().push(Rc::new(matcher_boolean));
    lexer.matchers_mut().push(Rc::new(matcher_identifier));
    lexer
}

pub struct Lexer {
    tokenizer: Tokenizer,
    matchers: Vec<Rc<Matcher>>,
}

#[allow(dead_code)]
impl Lexer {
    pub fn new(tokenizer: Tokenizer) -> Lexer {
        Lexer {
            tokenizer,
            matchers: Vec::new(),
        }
    }

    pub fn match_token(&mut self) -> Option<Token> {
        for matcher in &mut self.matchers {
            match self.tokenizer.try_match_token(matcher.as_ref()) {
                Some(t) => return Some(t),
                None => continue,
            }
        }
        None
    }

    pub fn matchers(&self) -> &Vec<Rc<Matcher>> {
        &self.matchers
    }

    pub fn matchers_mut(&mut self) -> &mut Vec<Rc<Matcher>> {
        &mut self.matchers
    }
}

impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        let token = match self.match_token() {
            Some(n) => n,
            None    => return None,
        };
        match token.token_type {
            TokenType::EOF => None,
            TokenType::Whitespace => {
                match self.next() {
                    Some(t) => Some(t),
                    None => None,
                }
            }
            _ => Some(token),
        }
    }
}
