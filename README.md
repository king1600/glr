# GLR
The Great Language RIIR (Rewrite It In Rust) is a toy language submitted
for a 2 month long programming challenge in the 
[Rust Discord](https://bit.ly/rust-community) under
[#langdev](https://discordapp.com/channels/273534239310479360/490356824420122645)

## Goals:

* **Rust-like** - An attempt is made to use syntax similar to rust.
* **Memory Consumption** - Attention will be payed to memory usage.
* **Object Oriented** - The structure is similar to oop langs such as Java or C#.
* **JIT Compiled** - Code should be compiled to machine code for execution when possible.
* **Statically Typed** - Types are known at compile time which aids in machine code generation.
* **Garbage Collected** - The goals include having a multi-threaded garbage collector similar to Java 11's ZGC.

## Example

```rust,csharp
use std.io as IO;

pub class Program {
    pub fn main(args: [String]) {
        IO.print(args)
    }
}
```