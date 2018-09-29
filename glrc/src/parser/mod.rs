#[allow(dead_code)]
pub mod token;

#[allow(dead_code)]
pub mod lexer;

pub type IResult<T> = Result<T, (Error, SourceLoc)>;

pub type SourceLoc = (usize, usize, usize);

pub enum Error {
    UnterminatedString,
}