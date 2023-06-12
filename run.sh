#!/bin/bash

export RUST_BACKTRACE=1

MAELSTROM="./maelstrom/maelstrom"
NODE="./maelstrom-rust/target/debug/maelstrom-rust"
#NODE="test.py"

function run() {
    workload="$1"
    shift;
    "$MAELSTROM" test -w "$workload" --bin "$NODE" --log-stderr --time-limit=10 "$@"
}


cmd="$1"
shift;

"$cmd" "$@"
