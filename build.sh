#!/bin/sh

features() {
	echo ext_wasm

	echo sum_std
	echo sum_wasm
	echo sum_simd

	echo cnt_std
	echo cnt_wasm
	echo cnt_simd

	echo mean_arithmetic
	echo mean_simd
	echo mean_arithmetic_std
	echo mean_arithmetic_wasm
	echo mean_arithmetic_simd
}

export RUSTFLAGS='-C target_feature=+simd128'
cargo \
	build \
	--target wasm32-unknown-unknown \
	--features $(features | tr '\n' , | sed 's/,$//') \
	--profile release-wasm
