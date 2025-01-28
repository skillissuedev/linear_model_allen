# Allen

Modern OpenAL bindings for Rust.

Inspired by the lack of development on the [alto](https://crates.io/crates/alto/) crate. This crate is very experimental and needs a lot of work still.

See examples folder for usage.

## Fork differences 

The only difference is that the linear distance model is default now

## Windows compilation

To compile this library for Windows, make sure you've installed OpenAL 1.1 
Core SDK in the default directory (C:\Program Files (x86)\OpenAL 1.1 SDK), otherwise your program will not compile.
