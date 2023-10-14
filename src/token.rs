use std::fmt::Display;

#[derive(PartialEq, Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub literal: String,
}

#[derive(PartialEq, Debug)]
pub enum TokenKind {
    Illegal,
    Eof,

    Ident,
    Int,

    Assign,
    Plus,

    Comma,
    Semicolon,

    LParen,
    RParen,
    LBrace,
    RBrace,

    Function,
    Let,
}

impl Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            TokenKind::Illegal => "Illegal",
            TokenKind::Eof => "EOF",
            TokenKind::Ident => "Ident",
            TokenKind::Int => "Int",
            TokenKind::Assign => "=",
            TokenKind::Plus => "+",
            TokenKind::Comma => ",",
            TokenKind::Semicolon => ";",
            TokenKind::LParen => "(",
            TokenKind::RParen => ")",
            TokenKind::LBrace => "{",
            TokenKind::RBrace => "}",
            TokenKind::Function => "fn",
            TokenKind::Let => "let",
        };
        write!(f, "{}", s)
    }
}