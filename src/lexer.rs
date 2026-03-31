use std::str::Chars;
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
    }, // '--[[...]]', '--[==[...]==]'
    
    // Literals
    /// number: '3', '3.0', '3.1416', '314.16e-2', '0.31416E1', '0xff', '0x56'
    /// string: 'string', "string", [[...]], [==[...]==]
    Literal { kind: LiteralKind },

    // Indentifier
    Ident,      // 'local', 'function', 'x', 'dy'

    // Symbols
    Plus,       // '+'
    Minus,      // '-'
    Star,       // '*'
    Slash,      // '/'
    Percent,    // '%'
    Caret,      // '^'
    Hash,       // '#'
    Eq,         // '='
    EqEq,       // '=='
    TildeEq,    // '~='
    Lt,         // '<'
    LtEq,       // '<='
    Gt,         // '>'
    GtEq,       // '>='
    LParen,     // '('
    RParen,     // ')'
    LBrace,     // '{'
    RBrace,     // '}'
    LBracket,   // '['
    RBracket,   // ']'
    Semi,       // ';'
    Colon,      // ':'
    Comma,      // ','
    Dot,        // '.'
    DotDot,     // '..'
    Ellipsis,   // '...'
    
    // Special
    Error,      // Error token
    Eof,        // End of file mark
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
    pub inner: Chars<'a>,
    pub len_remaining: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(inner: &'a str) -> Self {
        Self {
            inner: inner.chars(),
            len_remaining: inner.len(),
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
        let Some(cur) = self.consume() else {
            return Token::new(TokenKind::Eof, 0);
        };
        let kind = match cur {
            // skip whitespace
            c if is_whitespace(c) => {
                while is_whitespace(self.peek()) {
                    self.consume();
                }
                TokenKind::Whitespace
            }
            '0'..='9' => {
                self.consume_number(cur);
                TokenKind::Literal {
                    kind: LiteralKind::Number,
                }
            }
            '-' => {
                // comments
                if self.expect("-[") {
                    let start_level = self.len_remaining;
                    while self.expect("=") {}
                    let level = self.len_remaining - start_level;
                    if self.peek() == '[' {
                        while self.peek() != ']' {
                            self.consume();
                        }
                        let mut level_str = "]".to_string();
                        level_str.push_str(&"=".repeat(level).to_string());
                        level_str.push(']');
                        if self.expect(&level_str) {
                            TokenKind::BlockComment {
                                level: level as u32,
                            }
                        } else {
                            TokenKind::Error
                        }
                    } else {
                        TokenKind::Error
                    }
                } else if self.starts_with("-") {
                    while !matches!(self.peek(), '\n' | '\r') {
                        if self.consume().is_none() {
                            break;
                        };
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
                    let start_level = self.len_remaining;
                    while self.expect("=") {}
                    let level = self.len_remaining - start_level;
                    if self.peek() == '[' {
                        while self.peek() != ']' {
                            self.consume();
                        }
                        let mut level_str = "]".to_string();
                        level_str.push_str(&"=".repeat(level).to_string());
                        level_str.push(']');
                        if self.starts_with(level_str.as_str()) {
                            TokenKind::Literal {
                                kind: LiteralKind::LongString {
                                    level: level as u32,
                                },
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
        let tok = Token::new(kind, self.pos_within_token());
        self.reset_pos_within_token();
        tok
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
        self.inner.next().map(|cur| cur)
    }
    fn consume_bytes(&mut self, n: usize) {
        self.inner = self.inner.as_str()[n..].chars();
    }
    fn consume_while<F>(&mut self, mut cond: F)
    where
        F: FnMut(char) -> bool,
    {
        while cond(self.peek()) {
            self.consume();
        }
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
    fn pos_within_token(&self) -> u32 {
        (self.len_remaining - self.inner.as_str().len()) as u32
    }
    fn reset_pos_within_token(&mut self) {
        self.len_remaining = self.inner.as_str().len()
    }

    fn consume_number(&mut self, first: char) {
        if first == '0' && matches!(self.peek(), 'x' | 'X') {
            self.consume();
            self.consume_while(is_hex_digit);
            return;
        }

        self.consume_while(is_number);

        if self.peek() == '.' && self.inner.as_str().as_bytes().get(1).copied() != Some(b'.') {
            self.consume();
            self.consume_while(is_number);
        }

        if matches!(self.peek(), 'e' | 'E') {
            self.consume();
            if matches!(self.peek(), '+' | '-') {
                self.consume();
            }
            self.consume_while(is_number);
        }
    }
}

fn is_whitespace(c: char) -> bool {
    matches!(c, ' ' | '\t' | '\n' | '\r')
}

fn is_number(c: char) -> bool {
    matches!(c, '0'..='9')
}

fn is_hex_digit(c: char) -> bool {
    matches!(c, '0'..='9' | 'a'..='f' | 'A'..='F')
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use unindent::unindent;
    #[test]
    fn trivias_linecomment() {
        let tokens = Lexer::new("--hello").tokenize();
        assert_eq!(tokens, vec![Token::new(TokenKind::LineComment, 7),]);
    }
    #[test]
    fn trivias_blockcomment() {
        // no level
        let target = unindent(
            r#"
            --[[
            hello
            ]]
            "#,
        );
        let mut lexer = Lexer::new(target.as_str());
        let tokens = lexer.tokenize();
        assert_eq!(
            tokens,
            vec![
                Token::new(TokenKind::BlockComment { level: 0 }, 13),
                Token::new(TokenKind::Whitespace, 1),
            ]
        );
        // with level
        let target = unindent(
            r#"
            --[==[
            hello
            ]==]
            "#,
        );
        let mut lexer = Lexer::new(target.as_str());
        let tokens = lexer.tokenize();
        assert_eq!(
            tokens,
            vec![
                Token::new(TokenKind::BlockComment { level: 2 }, 17),
                Token::new(TokenKind::Whitespace, 1),
            ]
        );
    }
    #[test]
    fn lieral_number() {
        let target = "3";
        let mut lexer = Lexer::new(target);
        let tokens = lexer.tokenize();
        assert_eq!(
            tokens,
            vec![Token::new(
                TokenKind::Literal {
                    kind: LiteralKind::Number
                },
                1
            ),]
        );
        let target = "-3";
        let mut lexer = Lexer::new(target);
        let tokens = lexer.tokenize();
        assert_eq!(
            tokens,
            vec![
                Token::new(TokenKind::Minus, 1),
                Token::new(
                    TokenKind::Literal {
                        kind: LiteralKind::Number
                    },
                    1
                ),
            ]
        );
        let target = "30";
        let mut lexer = Lexer::new(target);
        let tokens = lexer.tokenize();
        assert_eq!(
            tokens,
            vec![Token::new(
                TokenKind::Literal {
                    kind: LiteralKind::Number
                },
                2
            ),]
        );
        let target = "3.1416";
        let mut lexer = Lexer::new(target);
        let tokens = lexer.tokenize();
        assert_eq!(
            tokens,
            vec![Token::new(
                TokenKind::Literal {
                    kind: LiteralKind::Number
                },
                6
            ),]
        );
        let target = "314.16e-2";
        let mut lexer = Lexer::new(target);
        let tokens = lexer.tokenize();
        assert_eq!(
            tokens,
            vec![Token::new(
                TokenKind::Literal {
                    kind: LiteralKind::Number
                },
                9
            ),]
        );
        let target = "0.31416E1";
        let mut lexer = Lexer::new(target);
        let tokens = lexer.tokenize();
        assert_eq!(
            tokens,
            vec![Token::new(
                TokenKind::Literal {
                    kind: LiteralKind::Number
                },
                9
            ),]
        );
        let target = "0x5f6";
        let mut lexer = Lexer::new(target);
        let tokens = lexer.tokenize();
        assert_eq!(
            tokens,
            vec![Token::new(
                TokenKind::Literal {
                    kind: LiteralKind::Number
                },
                5
            ),]
        );
    }
}
