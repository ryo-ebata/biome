#!/bin/sh

curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh 
pnpm build:wasm
pnpm install
pnpm build:js