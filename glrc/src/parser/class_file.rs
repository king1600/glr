use super::{Span, Parser};
use super::expression::Expr;
use super::typing::{Type, TypeGuard};

#[derive(Debug)]
pub enum ClassFile<'a> {
    Enum(Enum<'a>),
    Trait(Trait<'a>),
    Struct(Struct<'a>),
}

#[derive(Debug)]
pub struct Struct<'a> {
    pub inner: Trait<'a>,
    pub fields: Vec<Field<'a>>,
}

#[derive(Debug)]
pub struct Enum<'a> {
    pub inner: Trait<'a>,
    pub types: Vec<EnumType<'a>>,
}

#[derive(Debug)]
pub struct Trait<'a> {
    pub access: u8,
    pub name: Type<'a>,
    pub funcs: Vec<FuncDef<'a>>,
    pub guards: Vec<TypeGuard<'a>>,
}

#[derive(Debug)]
pub struct FuncDef<'a> {
    pub ret: Type<'a>,
    pub name: Type<'a>,
    pub args: VarDef<'a>,
    pub guards: Vec<TypeGuard<'a>>,
    pub body: Option<Vec<Expr<'a>>>,
}

pub type Field<'a> = (u8, Type<'a>, VarDef<'a>);

pub type EnumType<'a> = (&'a str, Vec<Type<'a>>);

pub type VarDef<'a> = Vec<(&'a str, Option<Expr<'a>>)>;

#[inline(always)] // just for testing
fn empty<T>() -> T {
    use std::mem::uninitialized;
    unsafe { uninitialized::<T>() }
}

impl_parser!(ClassFile<'a>, ClassFile<'a>, alt!(
    Enum<'a>::parse |
    Trait<'a>::parse |
    Struct<'a>:: parse
));

impl_parser!(Enum<'a>, ClassFile<'a>, do_parse!(
    _x: tag!("enum") >>
    (ClassFile::Enum(empty::<Enum<'a>>()))
));

impl_parser!(Trait<'a>, ClassFile<'a>, do_parse!(
    _x: tag!("trait") >>
    (ClassFile::Trait(empty::<Trait<'a>>()))
));


impl_parser!(Struct<'a>, ClassFile<'a>, do_parse!(
    _x: tag!("struct") >>
    (ClassFile::Struct(empty::<Struct<'a>>()))
));