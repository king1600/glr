pub type SourceLoc = (usize, usize);

pub type Expr<'a> = (ExprType<'a>, SourceLoc);

pub type TypeGuard<'a> = (&'a str, Vec<Type<'a>>);

pub type VarDef<'a> = (&'a str, Option<Type<'a>>, Option<Expr<'a>>);

#[derive(Debug)]
pub struct ClassFile<'a> {
    pub(super) imports: Vec<Using<'a>>,
    pub(super) class: ClassDef<'a>,
}

#[derive(Debug)]
pub enum ExprType<'a> {
    Int(u64),
    Float(f64),
    Id(&'a str),
    Str(&'a str),
    Break,
    Continue,
    Return(Box<Expr<'a>>),
    Func(FuncDef<'a>),
    Class(ClassDef<'a>),
    Var(Vec<VarDef<'a>>),
    Field(i32, Box<Expr<'a>>),
    Unop(Operator, Box<Expr<'a>>),
    Index(Box<(Expr<'a>, Expr<'a>)>),
    Call(Box<Expr<'a>>, Vec<Expr<'a>>),
    Binop(Operator, Box<(Expr<'a>, Expr<'a>)>),
    If(Vec<(Expr<'a>, Vec<Expr<'a>>)>, Option<Vec<Expr<'a>>>),
}

#[derive(Debug)]
pub enum Type<'a> {
    Int32,
    Int64,
    Float32,
    Float64,
    Object(&'a str),
    Array(Box<Type<'a>>),
    Generic(&'a str, Vec<Type<'a>>),
}

#[derive(Debug)]
pub enum Using<'a> {
    Module(&'a str, Option<&'a str>),
    Include(&'a str, Vec<Using<'a>>),
}

#[derive(Debug)]
pub enum ClassType<'a> {
    Enum(Type<'a>, Vec<TypeGuard<'a>>),
    Trait(Type<'a>, Vec<Type<'a>>, Vec<TypeGuard<'a>>),
    Struct(Type<'a>, Vec<Type<'a>>, Vec<TypeGuard<'a>>),
}

#[derive(Debug)]
pub enum FieldDef<'a> {
    Field(i32, VarDef<'a>),
    Enum(&'a str, Vec<Type<'a>>),
}

#[derive(Debug)]
pub struct FuncDef<'a> {
    pub(super) access: i32,
    pub(super) ret: Type<'a>,
    pub(super) name: Type<'a>,
    pub(super) args: Vec<VarDef<'a>>,
    pub(super) body: Option<Vec<Expr<'a>>>,
    pub(super) guards: Option<Vec<TypeGuard<'a>>>,
}

#[derive(Debug)]
pub struct ClassDef<'a> {
    pub(super) access: i32,
    pub(super) ctype: ClassType<'a>,
    pub(super) fields: Vec<FieldDef<'a>>,
    pub(super) methods: Vec<FuncDef<'a>>,
}

pub mod access {
    pub const PUBLIC: i32 = 1 << 0;
    pub const CONST:  i32 = 1 << 1;
    pub const STATIC: i32 = 1 << 2;

    #[inline]
    pub fn from(modifier: &str) -> i32 {
        match modifier {
            "pub" => PUBLIC,
            "const" => CONST,
            "static" => STATIC,
            _ => 0,
        }
    }
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

impl<'a> ExprType<'a> {
    #[inline]
    pub fn binop(op: Operator, lhs: Expr<'a>, rhs: Expr<'a>) -> Expr<'a> {
        let (_, (_, end)) = rhs;
        let (_, (start, _)) = lhs;
        (ExprType::Binop(op, Box::new((lhs, rhs))), (start, end))
    }
}