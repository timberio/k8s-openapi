#!/bin/bash

set -euo pipefail

./ci/rustup.sh

case "$OP" in
	'clippy')
		pushd k8s-openapi-derive-impl
		RUST_BACKTRACE=full cargo build --release --target wasm32-unknown-unknown
		popd

		pushd k8s-openapi-derive
		cargo clippy
		popd
		;;

	'clippy-impl')
		pushd k8s-openapi-derive-impl
		cargo clippy
		popd
		;;

	*)
		exit 1
		;;
esac
