#!/usr/bin/sh
#cargo test
cargo build --release
strip target/release/cw
