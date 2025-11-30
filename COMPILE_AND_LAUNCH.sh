#!/usr/bin/env bash

echo "INSTALLING + IDENTIFYING RUST COMPILATION TARGET (<architecture>-<vendor>-<operating_system/environment>)"
echo "================================================"
rustup target add wasm32-unknown-unknown  # unknown vendor (PC/Apple), unknown OS (it's just 32-bit WebAsm)

echo "INSTALLING BINDING GENERATOR"
echo "============================"
cargo install wasm-bindgen-cli
# wasm-bindgen --version

echo "BUILDING"
echo "========"
cargo build --release --target wasm32-unknown-unknown  # produces: target/wasm32-unknown-unknown/release/wasm_wordle.wasm

echo "WASM-BINDGEN (JS GLUE + REFERENCEABLE WASM)"
echo "==========================================="
mkdir -p web
wasm-bindgen --target web --out-dir ./web  'target/wasm32-unknown-unknown/release/wasm_wordle.wasm'  # produces: web/wasm_wordle.js , web/wasm_wordle_bg.wasm

echo "COPYING RESOURCES INTO web/"
echo "==========================="
cp static/index.html web/  # will be our webpage, needs to be at root
cp assets/*_*-char_wordlist.txt web/assets/
# cp -r static web/
# cp -r assets web/

echo "RUN ON LOCAL WEB SERVER (using Python3)"
echo "======================="
echo '**************************************************************'
echo 'PRESS "CTRL-C"/"COMMAND-C" IN THIS SHELL TO TERMINATE HOSTING'
echo '**************************************************************'
env --chdir=web/ python3 -m http.server 8080  # opens: http://localhost:8080
# Node alternative:  npm install -g serve && serve web

