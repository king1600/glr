use nom::types::CompleteStr;
use nom_locate::LocatedSpan;

pub type P<'a, T> = (T, Span<'a>);
pub type Span<'a> = LocatedSpan<CompleteStr<'a>>;

pub trait ToSpan<'a> {
    fn to_span(self) -> Span<'a>;
}

impl<'a> ToSpan<'a> for &'a str {
    fn to_span(self) -> Span<'a> {
        Span::new(CompleteStr(self))
    }
}

pub trait Parser<'a, T: Sized> {
    fn parse(input: Span<'a>) -> nom::IResult<Span<'a>, T>;
}

#[macro_export]
macro_rules! impl_parser {
    ($name:ty, $ty:ty, $($body:tt)*) => {
        impl<'a> Parser<'a, $ty> for $name {
            named!(parse<Span<'a>, $ty>, $($body)*);
        }
    };
}

#[allow(dead_code)]
pub mod typing;

#[allow(dead_code)]
pub mod class_file;

#[allow(dead_code)]
pub mod expression;