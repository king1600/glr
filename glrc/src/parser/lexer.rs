use super::*;
use super::token::*;
use self::Operator::*;
use self::TokenType::*;

use std::iter::Peekable;
use std::str::CharIndices;
use std::collections::HashMap;

pub struct Lexer<'a> {
    pos: usize,
    loc: SourceLoc,
    stream: Peekable<CharIndices<'a>>,
}

impl<'a> From<&'a str> for Lexer<'a> {
    fn from(source: &'a str) -> Lexer<'a> {
        Lexer {
            pos: 0,
            loc: (1, 0, 0),
            stream: source.char_indices().peekable(),
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = IResult<Token<'a>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.skip_whitespace() {
            return None
        }

        let pos = self.loc.clone();
        match (self.next_char(), self.peek_char()) {
            (Some('&'), Some('=')) => self.eat(pos, Op(BitAnd, true)),
            (Some('~'), Some('=')) => self.eat(pos, Op(BitNot, true)),
            (Some('|'), Some('=')) => self.eat(pos, Op(BitOr, true)),
            (Some('>'), Some('>')) => self.eat(pos, Op(Shr, false)),
            (Some('<'), Some('<')) => self.eat(pos, Op(Shr, false)),
            (Some('+'), Some('+')) => self.eat(pos, Op(Inc, false)),
            (Some('-'), Some('-')) => self.eat(pos, Op(Dec, false)),
            (Some('&'), Some('&')) => self.eat(pos, Op(And, false)),
            (Some('>'), Some('=')) => self.eat(pos, Op(Gte, false)),
            (Some('<'), Some('=')) => self.eat(pos, Op(Lte, false)),
            (Some('|'), Some('|')) => self.eat(pos, Op(Or, false)),
            (Some('+'), Some('=')) => self.eat(pos, Op(Add, true)),
            (Some('-'), Some('=')) => self.eat(pos, Op(Sub, true)),
            (Some('*'), Some('=')) => self.eat(pos, Op(Mul, true)),
            (Some('/'), Some('=')) => self.eat(pos, Op(Div, true)),
            (Some('%'), Some('=')) => self.eat(pos, Op(Mod, true)),
            (Some('^'), Some('=')) => self.eat(pos, Op(Xor, true)),
            _ => None
        }
    }
}

impl<'a> Lexer<'a> {
    #[inline]
    fn eat(&mut self, pos: SourceLoc, token: TokenType<'a>) -> Option<IResult<Token<'a>>> {
        self.next_char();
        Some(Ok((token, pos)))
    }

    #[inline]
    fn peek_char(&mut self) -> Option<char> {
        self.stream.peek().and_then(|&(_, x)| Some(x))
    }

    fn next_char(&mut self) -> Option<char> {
        self.stream.next().and_then(|(pos, character)| {
            self.loc.0 += 1;
            self.pos += pos;
            if c == '\n' {
                self.loc.2 = self.pos;
                self.loc.0 += 1;
                self.loc.1 = 0;
            }
            Some(character)
        })
    }

    fn read_while<P>(&mut self, predicate: P) -> usize where P: Fn(char) -> bool {
        let mut consumed = 0;
        while let Some(character) = self.peek_char() {
            if predicate(character) {
                self.next_char();
                consumed += 1;
            } else {
                break
            }
        }
        consumed
    }
}