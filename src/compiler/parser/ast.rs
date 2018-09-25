pub type SourceLoc = (usize, usize);

pub type Expr<'a> = (ExprType<'a>, SourceLoc);

#[derive(Debug)]
pub enum Type<'a> {
    Int32,
    Int64,
    Float32,
    Float64,
    Object(&'a str),
    Generic(&'a str, Vec<Type<'a>>),
}

#[derive(Debug)]
pub enum ExprType<'a> {
    Int(u64),
    Float(f64),
    Id(&'a str),
    Str(&'a str),
    Field(i32, Box<Expr<'a>>),
    Unop(Operator, Box<Expr<'a>>),
    Call(Box<Expr<'a>>, Vec<Expr<'a>>),
    Binop(Operator, Box<(Expr<'a>, Expr<'a>)>),
    Class(&'a str, Vec<Type<'a>>, Vec<Expr<'a>>),
    Var(Type<'a>, Vec<(&'a str, Option<Box<Expr<'a>>>)>),
    Func(i32, Type<'a>, &'a str, Vec<Expr<'a>>, Vec<Expr<'a>>),
}

#[derive(Debug)]
pub enum Operator {
    Add,
    Sub,
    Div,
    Mul,
    Mod,
    Set,
    Shr,
    Shl,
    Xor,
    BitOr,
    BitAnd,
    BitNot,
    Equ,
    Neq,
    Gt,
    Gte,
    Lt,
    Lte,
    And,
    Or,
    Not,
}