
macro_rules! asm_func {
    ($name:ident, $asm:expr) => {
        extern "C" { pub fn $name() -> u64; }
        global_asm!(concat!(
            ".intel_syntax noprefix\n",
            concat!(".global ", stringify!($name), "\n"),
            concat!(stringify!($name), ": \n"),
            $asm
        ));
    };
}

asm_func!(interpret, r#"
    mov rax, 1
    ret
"#);