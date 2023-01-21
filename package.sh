#!/bin/bash

rm -rf build
mkdir -p build/bin

pnpm run build
cp -r dist build/

cargo build --manifest-path backend/Cargo.toml
cp ./backend/target/debug/controller-tools build/bin/backend

cp package.json build/package.json
cp plugin.json build/plugin.json
cp main.py build/main.py
cp README.md build/README.md
cp LICENSE build/LICENSE