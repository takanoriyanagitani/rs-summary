#!/bin/sh

features() {
	echo ext_wasm

	echo sum_std
	echo sum_wasm
	echo sum_simd

	echo cnt_std
	echo cnt_wasm
	echo cnt_simd
}

export RUSTFLAGS='-C target_feature=+simd128'
cargo \
	build \
	--target wasm32-unknown-unknown \
	--features $(features | tr '\n' , | sed 's/,$//') \
	--profile release-wasm
