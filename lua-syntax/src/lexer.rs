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
///    range: Range{start: 21, end: 22}
/// }
use std::ops::Range;
#[rustfmt::skip]
#[derive(Debug, Clone, Copy, PartialEq)]
enum TokenKind {
    // Trivias
    Whitespace,             // ' ', '\t', '\n'
    LineComment,            // '--...'
    BlockComment,           // '--[[...]]', '--[==[...]==]'
    
    // Literals
    Number,                 // '3', '3.0', '3.1416', '314.16e-2', '0.31416E1', '0xff', '0x56'
    String,                 // 'string', "string"
    LongString,             // '--[[...]]', '--[==[...]==]'

    // Indentifier
    Ident,                  // 'x', 'dy'

    // Reserved keywords
    True,                   // 'true'
    False,                  // 'false'
    Nil,                    // 'nil'
    And,                    // 'and'
    Or,                     // 'or'
    Not,                    // 'not'
    For,                    // 'for'
    While,                  // 'while'
    Until,                  // 'until'
    Repeat,                 // 'repeat'
    Do,                     // 'do'
    In,                     // 'in'
    If,                     // 'if'
    Then,                   // 'then'
    ElseIf,                 // 'elseif'
    Else,                   // 'else'
    Break,                  // 'break'
    End,                    // 'end'
    Return,                 // 'return'
    Local,                  // 'local'
    Function,               // 'function'

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

type TokenIdx = u64;
struct Token {
    pub kind: TokenKind,
    pub range: std::ops::Range<TokenIdx>,
}

pub struct Lexer<'a> {
    inner: &'a str,
    pos: TokenIdx,
    token_start: TokenIdx,
    token_end: TokenIdx,
}

impl<'a> Lexer<'a> {
    pub fn new(inner: &'a str) -> Self {
        Self {
            inner,
            pos: 0,
            token_start: 0,
        }
    }
    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        loop {
            let tok = self.next_token();
            let done = tok.kind == TokenKind::Eof;
            tokens.push(tok);
            if done {
                break;
            }
        }
        tokens
    }
    fn next_token(&mut self) -> Token {
        if self.is_eof() {
            Token {
                kind: TokenKind::Eof,
                range: Range {
                    start: self.pos,
                    end: self.pos,
                },
            }
        } else {
            self.scan_token()
        }
    }
    fn start_token(&mut self) {
        self.token_start = self.pos;
    }
    fn end_token(&mut self) {
        self.token_start = self.pos;
    }
    fn scan_token(&mut self) -> Token {
        match self.peek() {
            c @ b' ' | b'\n' | b'\t' | b'\r' => {
                while self.consume(iswhitespace)
                self.consume();
            }
        }
    }
    // peek n bytes ahead
    fn nth(&self, n: usize) -> u8 {
        let pos = usize::saturating_add(self.pos, n);
        self.inner.as_bytes().get(pos).copied().unwrap_or(b'0')
    }
    fn peek(&self) -> u8 {
        self.nth(0)
    }
    fn is_eof(&self) -> bool {
        self.pos >= self.inner.len()
    }
    fn is_whitespace(&self) -> bool {
        matches!(self.peek(), b' ' | b'\n' | b'\t' | b'\r')
    }
    fn skip(&mut self, n: usize) {
        usize::saturating_add(self.pos, n);
    }
    fn consume(&mut self, expect: &[u8]) -> bool {
        unimplemented!()
    }
}
