#!/bin/bash

#rustc hello_cargo/src/main.rs
#./main

#rustup update

<<com
cargo run
cargo run --example ping
    Finished dev [unoptimized + debuginfo] target(s) in 35m 02s
     Running `/workspace/CORDS/hello_cargo/target/debug/rust-libp2p-tutorial`
Local peer id: PeerId("12D3KooWBgtEDEgpRsJde4QAxsuUCNx1hky3dFQRixZvHB9gbAAh")
com

ROCKET_ENV=prod cargo run