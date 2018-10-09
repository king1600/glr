# GLR
The Great Language RIIR (Rewrite It In Rust) is a toy language submitted
for a 2 month long programming challenge in the 
[Rust Discord](https://bit.ly/rust-community) under
[#langdev](https://discordapp.com/channels/273534239310479360/490356824420122645).
This repo hosts the virtual machine (glr) which executes Great Language Bytecode (glb).
It was originaly planned to be written in rust, but that slowed down development so switched to C.

## Goals:

* **Low Memory** - Much attention is payed to memory usage.
* **Garbage Collected** - GC will be based on Java 11's ZGC for scalable heaps.
* **JIT Compiled** - Code should be compiled to machine code for execution when possible.

## Building

The code requires python to build because I :

* couldn't figure out how to support recursive sources in make
* and couldn't figure out how to change cmake compiler and windows caching

```
python3 build.py release
```