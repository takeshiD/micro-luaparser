/// local x = "12"
/// local M = {}
/// M.add = function(x,y)
///     return x+y
/// end
/// ->
/// [
/// Local, WSpace, Ident('x'), WSpace, Eq, WSpace, String, WSpace('\n'),
/// Local, WSpace, Ident('M'), WSpace, Eq, WSpace, LBrace, RBrace, WSpace('\n'),
/// Ident('M'), Dot, Ident("add"), WSpace, Eq, Function, LParen, Ident('x'), Comma, Ident('y'), RParen, WSpace('\n'),
/// WSpace(' '), Return, WSpace, Ident('x'), Plus, Ident('y'), WSpace('\n'),
/// End,WSpace('\n'),
/// Eof
/// ]
/// Ident('M') = Token {
///    kind: TokenKind::Ident
///    len: 1
/// }
use std::str::Chars;
#[derive(Debug, Clone, Copy, PartialEq)]
enum LiteralKind {
    Number,
    String,
    LongString { level: u32 },
    Boolean,
    Nil,
}
#[rustfmt::skip]
#[derive(Debug, Clone, Copy, PartialEq)]
enum TokenKind {
    // Trivias
    Whitespace,             // ' ', '\t', '\n', '\r'
    LineComment,            // '--...'
    BlockComment {
        level: u32,
    },           // '--[[...]]', '--[==[...]==]'
    
    // Literals
    /// number: '3', '3.0', '3.1416', '314.16e-2', '0.31416E1', '0xff', '0x56'
    /// string: 'string', "string", [[...]], [==[...]==]
    Literal { kind: LiteralKind },

    // Indentifier
    Ident,                  // 'local', 'function', 'x', 'dy'

    // Symbols
    Plus,                   // '+'
    Minus,                  // '-'
    Star,                   // '*'
    Slash,                  // '/'
    Percent,                // '%'
    Caret,                  // '^'
    Hash,                   // '#'
    Eq,                     // '='
    EqEq,                   // '=='
    TildeEq,                // '~='
    Lt,                     // '<'
    LtEq,                   // '<='
    Gt,                     // '>'
    GtEq,                   // '>='
    LParen,                 // '('
    RParen,                 // ')'
    LBrace,                 // '{'
    RBrace,                 // '}'
    LBracket,               // '['
    RBracket,               // ']'
    Semi,                   // ';'
    Colon,                  // ':'
    Comma,                  // ','
    Dot,                    // '.'
    DotDot,                 // '..'
    Ellipsis,               // '...'
    
    // Special
    Error,                  // Error token
    Eof,                    // End of file mark
}

#[derive(Debug, Clone, PartialEq)]
struct Token {
    pub kind: TokenKind,
    pub len: u32, // 4GB
}

impl Token {
    pub fn new(kind: TokenKind, len: u32) -> Self {
        Self { kind, len }
    }
}

pub struct Lexer<'a> {
    inner: Chars<'a>,
    pos: u32, // 4GB
}

impl<'a> Lexer<'a> {
    pub fn new(inner: &'a str) -> Self {
        Self {
            inner: inner.chars(),
            pos: 0,
        }
    }
    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        loop {
            let tok = self.next_token();
            if tok.kind == TokenKind::Eof {
                break;
            }
            tokens.push(tok);
        }
        tokens
    }
    fn next_token(&mut self) -> Token {
        let start = self.pos;
        let Some(cur) = self.consume() else {
            return Token::new(TokenKind::Eof, 0);
        };
        let kind = match cur {
            // skip whitespace
            ' ' | '\t' | '\n' | '\r' => {
                while matches!(self.peek(), ' ' | '\t' | '\n' | '\r') {
                    self.consume();
                }
                TokenKind::Whitespace
            }
            '-' => {
                // comments
                if self.starts_with("-[") {
                    let start_level = self.pos;
                    while self.expect("=") {}
                    let level = self.pos - start_level;
                    if self.peek() == '[' {
                        while self.peek() != ']' {
                            self.consume();
                        }
                        self.consume();
                        let mut level_str = "=".repeat(level as usize).to_string();
                        level_str.push(']');
                        if level > 0 && self.starts_with(level_str.as_str()) {
                            TokenKind::BlockComment { level }
                        } else {
                            TokenKind::Error
                        }
                    } else {
                        TokenKind::Error
                    }
                } else if self.starts_with("-") {
                    while matches!(self.peek(), '\n' | '\r') {
                        self.consume();
                    }
                    TokenKind::LineComment
                } else {
                    TokenKind::Minus
                }
            }
            '(' => TokenKind::LParen,
            ')' => TokenKind::RParen,
            '{' => TokenKind::LBrace,
            '}' => TokenKind::RBrace,
            '[' => match self.peek() {
                '[' => {
                    let start_level = self.pos;
                    while self.expect("=") {}
                    let level = self.pos - start_level;
                    if self.peek() == '[' {
                        while self.peek() != ']' {
                            self.consume();
                        }
                        self.consume();
                        let mut level_str = "=".repeat(level as usize).to_string();
                        level_str.push(']');
                        if level > 0 && self.starts_with(level_str.as_str()) {
                            TokenKind::Literal {
                                kind: LiteralKind::LongString { level },
                            }
                        } else {
                            TokenKind::Error
                        }
                    } else {
                        TokenKind::Error
                    }
                }
                _ => TokenKind::LBracket,
            },
            ']' => TokenKind::RBracket,
            '+' => TokenKind::Plus,
            '*' => TokenKind::Star,
            '/' => TokenKind::Slash,
            '%' => TokenKind::Percent,
            '^' => TokenKind::Caret,
            '#' => TokenKind::Hash,
            ';' => TokenKind::Semi,
            ':' => TokenKind::Colon,
            ',' => TokenKind::Comma,
            '.' => {
                if self.starts_with("..") {
                    TokenKind::Ellipsis
                } else if self.starts_with(".") {
                    TokenKind::DotDot
                } else {
                    TokenKind::Dot
                }
            }
            '=' => match self.peek() {
                '=' => {
                    self.consume();
                    TokenKind::EqEq
                }
                _ => TokenKind::Eq,
            },
            '<' => match self.peek() {
                '=' => {
                    self.consume();
                    TokenKind::LtEq
                }
                _ => TokenKind::Lt,
            },
            '>' => match self.peek() {
                '=' => {
                    self.consume();
                    TokenKind::GtEq
                }
                _ => TokenKind::Gt,
            },
            '~' => match self.peek() {
                '=' => TokenKind::TildeEq,
                _ => TokenKind::Error,
            },
            _ => TokenKind::Error,
        };
        Token::new(kind, self.pos - start)
    }

    // peek 1 byte
    fn peek(&self) -> char {
        self.inner
            .clone()
            .peekable()
            .peek()
            .cloned()
            .unwrap_or('\0')
    }
    // consume 1 byte
    fn consume(&mut self) -> Option<char> {
        self.inner.next()
    }
    fn consume_bytes(&mut self, n: usize) {
        self.inner = self.inner.as_str()[n..].chars();
    }
    // consume bytes if pattern is matched
    fn expect(&mut self, pattern: &str) -> bool {
        if self.starts_with(pattern) {
            self.consume_bytes(pattern.len());
            true
        } else {
            false
        }
    }
    fn starts_with(&self, pattern: &str) -> bool {
        self.inner.as_str().starts_with(pattern)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn symbols() {
        let target = "--hello";
        let tokens = Lexer::new(target).tokenize();
        assert_eq!(tokens, vec![Token::new(TokenKind::Ident, 4)]);
    }
}
