use super::*;
use super::token::*;
use self::Keyword::*;
use self::Operator::*;
use self::TokenType::*;

use std::iter::Peekable;
use std::str::CharIndices;
use std::collections::HashMap;

lazy_static! {
    static ref KEYWORDS: HashMap<&'static str, Keyword> = {
        let mut keywords = HashMap::new();
        keywords.insert("enum",   Enum);
        keywords.insert("trait",  Trait);
        keywords.insert("struct", Struct);
        keywords.insert("module", Module);
        keywords.insert("fn",     Func);
        keywords.insert("if",     If);
        keywords.insert("elif",   Elif);
        keywords.insert("else",   Else);
        keywords.insert("pub",    Pub);
        keywords.insert("const",  Const);
        keywords.insert("static", Static);
        keywords
    };
}

pub struct Lexer<'a> {
    pos: usize,
    loc: SourceLoc,
    source: &'a str,
    stream: Peekable<CharIndices<'a>>,
}

impl<'a> From<&'a str> for Lexer<'a> {
    fn from(source: &'a str) -> Lexer<'a> {
        Lexer {
            pos: 0,
            loc: (1, 0, 0),
            source: source,
            stream: source.char_indices().peekable(),
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = IResult<Token<'a>>;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.skip_whitespace() {
            return None
        }

        let pos = self.loc.clone();
        match (self.next_char(), self.peek_char()) {
            (Some('"'), _) => self.eat_string(pos),
            (Some('/'), Some('/')) => self.line_comment(),
            (Some('/'), Some('*')) => self.block_comment(),
            (Some('0'), Some('b')) => self.eat_num(pos, 2),
            (Some('0'), Some('B')) => self.eat_num(pos, 16),
            (Some('0'), Some('x')) => self.eat_num(pos, 16),
            (Some('0'), Some('X')) => self.eat_num(pos, 16),
            (Some(c), _) if Self::is_ident_start(c) => self.eat_ident(pos),
            (Some('.'), Some(c)) if c.is_digit(10) => self.eat_num(pos, 10),
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
            (Some('-'), Some('>')) => Some(Ok((SArrow, pos))),
            (Some('='), Some('>')) => Some(Ok((Arrow, pos))),
            (Some('&'), _) => Some(Ok((Op(BitAnd, false), pos))),
            (Some('~'), _) => Some(Ok((Op(BitNot, false), pos))),
            (Some('|'), _) => Some(Ok((Op(BitOr, false), pos))),
            (Some('+'), _) => Some(Ok((Op(Add, false), pos))),
            (Some('-'), _) => Some(Ok((Op(Sub, false), pos))),
            (Some('*'), _) => Some(Ok((Op(Mul, false), pos))),
            (Some('/'), _) => Some(Ok((Op(Div, false), pos))),
            (Some('%'), _) => Some(Ok((Op(Mod, false), pos))),
            (Some('!'), _) => Some(Ok((Op(Not, false), pos))),
            (Some('='), _) => Some(Ok((Op(Set, false), pos))),
            (Some('^'), _) => Some(Ok((Op(Xor, false), pos))),
            (Some('>'), _) => Some(Ok((Op(Gt, false), pos))),
            (Some('<'), _) => Some(Ok((Op(Lt, false), pos))),
            (Some('('), _) => Some(Ok((LParen, pos))),
            (Some(')'), _) => Some(Ok((RParen, pos))),
            (Some('['), _) => Some(Ok((LBrace, pos))),
            (Some(']'), _) => Some(Ok((RBrace, pos))),
            (Some('{'), _) => Some(Ok((LCurly, pos))),
            (Some('}'), _) => Some(Ok((RCurly, pos))),
            (Some(','), _) => Some(Ok((Comma, pos))),
            (Some(';'), _) => Some(Ok((Semi, pos))),
            (Some('.'), _) => Some(Ok((Dot, pos))),
            _ => None
        }
    }
}

impl<'a> Lexer<'a> {
    #[inline]
    fn is_ident_start(character: char) -> bool {
        character.is_alphabetic() || character == '_' || character == '$'
    }

    #[inline]
    fn skip_whitespace(&mut self) -> bool {
        self.read_while(|c| c.is_whitespace());
        self.peek_char().is_some()
    }

    #[inline]
    fn eat(&mut self, pos: SourceLoc, token: TokenType<'a>) -> Option<IResult<Token<'a>>> {
        self.next_char();
        Some(Ok((token, pos)))
    }

    #[inline(always)]
    fn error(&self, pos: SourceLoc, error: Error) -> Option<IResult<Token<'a>>> {
        Some(Err((error, pos)))
    }

    #[inline]
    fn peek_char(&mut self) -> Option<char> {
        self.stream.peek().and_then(|&(_, x)| Some(x))
    }

    fn next_char(&mut self) -> Option<char> {
        self.stream.next().and_then(|(pos, character)| {
            self.loc.0 += 1;
            self.pos += pos;
            if character == '\n' {
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

    fn line_comment(&mut self) -> Option<IResult<Token<'a>>> {
        self.next_char();
        self.read_while(|c| c != '\n');
        self.next()
    }

    fn block_comment(&mut self) -> Option<IResult<Token<'a>>> {
        self.next_char();

        let mut depth = 1;
        while depth != 0 {
            match (self.next_char(), self.next_char()) {
                (None, _) | (_, None) => return Some(Err((Error::UnterminatedComment, self.loc.clone()))),
                (Some('*'), Some('/')) => depth -= 1,
                (Some('/'), Some('*')) => depth += 1,
                _ => return None,
            }
        }

        self.next()
    }

    fn eat_string(&mut self, pos: SourceLoc) -> Option<IResult<Token<'a>>> {
        let start = self.pos;

        while self.read_while(|c| c != '"') > 0 {
            if self.source[self.pos - 1..].chars().next().unwrap_or('\0') != '\\' {
                break
            }
        }

        if self.next_char().unwrap_or('\0') != '"' {
            Some(Err((Error::UnterminatedString, pos)))
        } else {
            Some(Ok((Str(&self.source[start..self.pos - 1]), pos)))
        }
    }

    fn eat_ident(&mut self, pos: SourceLoc) -> Option<IResult<Token<'a>>> {
        let length = self.read_while(|c| Self::is_ident_start(c) || c.is_digit(10));
        let identifier = &self.source[self.pos - length..self.pos];
        let keyword = KEYWORDS.get(&identifier).and_then(|kw| Some(Kw(kw.clone())));
        Some(Ok((keyword.unwrap_or(Id(identifier)), pos)))
    }

    fn eat_num(&mut self, pos: SourceLoc, radix: i32) -> Option<IResult<Token<'a>>> {
        None
    }
}