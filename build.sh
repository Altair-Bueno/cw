#!/usr/bin/sh
cargo test
cargo build --release
strip target/release/cw

time target/release/cw test/resources/* > /dev/null
