#!/bin/bash
set -e

export RUST_BACKTRACE=1
_SCRIPT_PATH=$(readlink -f "${BASH_SOURCE:-$0}")
SCRIPT_DIR=$(dirname "$_SCRIPT_PATH")
SRC_DIR="$SCRIPT_DIR/maelstrom-rust"

MAELSTROM="$SCRIPT_DIR/.maelstrom/maelstrom"

function async-comm-service() {
	workload="$1"
	node="$SRC_DIR/target/debug/async-comm-service"
	shift
	pushd "$SRC_DIR"
	cargo build
	popd
	"$MAELSTROM" test -w "$workload" --bin "$node" --log-stderr --time-limit=20 "$@"
}

function crdt-service() {
	workload="$1"
	node="$SRC_DIR/target/debug/async-comm-service"
	shift
	pushd "$SRC_DIR"
	cargo build
	popd
	"$MAELSTROM" test -w "$workload" --bin "$node" --log-stderr --time-limit=20 "$@"
}

cmd="$1"
shift

"$cmd" "$@"
