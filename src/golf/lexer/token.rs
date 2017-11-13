use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    IntLiteral,
    FloatLiteral,
    StringLiteral,
    CharLiteral,
    BoolLiteral,
    Symbol,
    Operator,
    Identifier,
    Type,
    Keyword,
    Whitespace,
    Indent,
    EOL,
    EOF,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct TokenPosition {
    pub line: usize,
    pub col:  usize,
}

impl Default for TokenPosition {
    fn default() -> Self {
        TokenPosition {
            line: 1,
            col: 0,
        }
    }
}

impl fmt::Display for TokenPosition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}, {}]", self.line, self.col)
    }
}

impl TokenPosition {
    pub fn new(line: usize, col: usize) -> TokenPosition {
        TokenPosition {
            line, col,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub position:   TokenPosition,
    content:        String,
}

#[allow(dead_code)]
impl Token {
    pub fn new(token_type: TokenType, position: TokenPosition, content: String) -> Token {
        Token {
            token_type,
            position,
            content,
        }
    }

    pub fn content(&self) -> &String {
        &self.content
    }
}

impl PartialEq for Token {
    fn eq(&self, other: &Token) -> bool {
        self.token_type == other.token_type
    }
}
