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
    Minus,
    Bang,
    Asterisk,
    Slash,

    Lt,
    Gt,

    Eq,
    NotEq,

    Comma,
    Semicolon,

    LParen,
    RParen,
    LBrace,
    RBrace,

    Function,
    Let,
    True,
    False,
    If,
    Else,
    Return,
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
            TokenKind::Minus => "-",
            TokenKind::Bang => "!",
            TokenKind::Asterisk => "*",
            TokenKind::Slash => "/",
            TokenKind::Lt => "<",
            TokenKind::Gt => ">",
            TokenKind::Eq => "==",
            TokenKind::NotEq => "!=",
            TokenKind::Comma => ",",
            TokenKind::Semicolon => ";",
            TokenKind::LParen => "(",
            TokenKind::RParen => ")",
            TokenKind::LBrace => "{",
            TokenKind::RBrace => "}",
            TokenKind::Function => "fn",
            TokenKind::Let => "let",
            TokenKind::True => "true",
            TokenKind::False => "false",
            TokenKind::If => "if",
            TokenKind::Else => "else",
            TokenKind::Return => "return",
        };
        write!(f, "{}", s)
    }
}

pub fn lookup_ident(ident: &str) -> TokenKind {
    match ident {
        "fn" => TokenKind::Function,
        "let" => TokenKind::Let,
        "true" => TokenKind::True,
        "false" => TokenKind::False,
        "if" => TokenKind::If,
        "else" => TokenKind::Else,
        "return" => TokenKind::Return,
        _ => TokenKind::Ident,
    }
}