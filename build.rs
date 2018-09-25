extern crate peg;

fn main() {
    peg::cargo_build("src/compiler/parser/grammar.rustpeg");
}