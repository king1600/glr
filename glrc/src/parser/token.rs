use std::fmt;
use super::*;

pub type Token<'a> = (TokenType<'a>, SourceLoc);

#[derive(Copy, Clone, PartialEq)]
pub enum TokenType<'a> {
    Dot,
    Semi,
    Colon,
    Comma,
    Arrow,
    SArrow,
    LParen,
    RParen,
    LBrace,
    RBrace,
    LCurly,
    RCurly,
    Int(u64),
    Float(f64),
    Kw(Keyword),
    Id(&'a str),
    Str(&'a str),
    Op(Operator, bool),
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Keyword {
    Enum,
    Trait,
    Struct,
    Module,
    Func,
    If,
    Elif,
    Else,
    Pub,
    Const,
    Static,
}

#[derive(Copy, Clone, PartialEq)]
pub enum Operator {
    Inc,
    Dec,
    Add,
    Sub,
    Div,
    Mul,
    Mod,
    Set,
    Equ,
    Neq,
    Lt,
    Lte,
    Gt,
    Gte,
    And,
    Or,
    Not,
    Xor,
    Shr,
    Shl,
    BitOr,
    BitAnd,
    BitNot,
}

impl<'a> fmt::Debug for TokenType<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::TokenType::*;
        match self {
            Dot => write!(f, "."),
            Semi => write!(f, ";"),
            Colon => write!(f, ":"),
            Comma => write!(f, ","),
            Arrow => write!(f, "=>"),
            SArrow => write!(f, "->"),
            LParen => write!(f, "("),
            RParen => write!(f, ")"),
            LBrace => write!(f, "["),
            RBrace => write!(f, "]"),
            LCurly => write!(f, "{{"),
            RCurly => write!(f, "}}"),
            Id(x) => write!(f, "{}", x),
            Int(x) => write!(f, "{}", x),
            Kw(x) => write!(f, "{:?}", x),
            Float(x) => write!(f, "{}", x),
            Str(x) => write!(f, "\"{}\"", x),
            Op(x, true) => write!(f, "{:?}=", x),
            Op(x, false) => write!(f, "{:?}", x),
        }
    }
}

impl fmt::Debug for Operator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Operator::*;
        write!(f, "{}", match self {
            Inc => "--",
            Dec => "++",
            Add => "+",
            Sub => "-",
            Div => "/",
            Mul => "*",
            Mod => "%",
            Set => "=",
            Equ => "==",
            Neq => "!=",
            Lt => "<",
            Lte => "<=",
            Gt => ">",
            Gte => ">=",
            And => "&&",
            Or => "||",
            Not => "!",
            Xor => "^",
            Shr => ">>",
            Shl => "<<",
            BitOr => "|",
            BitAnd => "&",
            BitNot => "~",
        })
    }
}