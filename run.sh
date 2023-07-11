#!/bin/bash

export RUST_BACKTRACE=1
_SCRIPT_PATH=$(readlink -f "${BASH_SOURCE:-$0}")
SCRIPT_DIR=$(dirname "$_SCRIPT_PATH")
SRC_DIR="$SCRIPT_DIR/maelstrom-rust"

MAELSTROM="$SCRIPT_DIR/.maelstrom/maelstrom"
NODE="$SRC_DIR/target/debug/maelstrom-rust"

function run() {
    workload="$1"
    shift;
    pushd $SRC_DIR; cargo build; popd
    "$MAELSTROM" test -w "$workload" --bin "$NODE" --log-stderr --time-limit=20 "$@"
}


cmd="$1"
shift;

"$cmd" "$@"
