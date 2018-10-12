# GLR
The Great Language RIIR (Rewrite It In Rust) is a toy language submitted
for a 2 month long programming challenge in the 
[Rust Discord](https://bit.ly/rust-community) under
[#langdev](https://discordapp.com/channels/273534239310479360/490356824420122645).
This repo hosts both 
the virtual machine (glr) which executes Great Language Bytecode (glb),
and the language compiler (glrc) which compiles source code to glb.

## Goals:

* **Low Memory** - Much attention is payed to memory usage.
* **Garbage Collected** - GC will be based on Java 11's ZGC for scalable heaps.
* **JIT Compiled** - Code should be compiled to machine code for execution when possible.

## Building

For building the virtual machine:
```
cargo build -p glr --release
```

For building the compiler
```
cargo build -p glrc --release
```