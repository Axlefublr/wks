#!/usr/bin/env fish

test "$argv[1]" || return 1
cargo build --release --bin $argv[1]
and rsync ./target/release/$argv[1] ~/fes/eva/wks/$argv[1].rs
or read -n 1 -P ...
