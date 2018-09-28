
pub type TypeGuard<'a> = (&'a str, Vec<Type<'a>>);

#[derive(Debug)]
pub enum Type<'a> {
    Int32,
    Int64,
    Float32,
    Float64,
    Object(&'a str),
    Generic(&'a str, Vec<Type<'a>>),
}