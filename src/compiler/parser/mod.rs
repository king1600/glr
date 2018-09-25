#[allow(dead_code)]
pub mod ast;

mod grammar {
    include!(concat!(env!("OUT_DIR"), "/grammar.rs"));
}