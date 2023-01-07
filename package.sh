#!/bin/bash

rm -rf build
mkdir -p build/bin

npm run build
cp -r dist build/

cross build --manifest-path backend/Cargo.toml
cp ./backend/target/x86_64-unknown-linux-gnu/debug/controller-tools build/bin/backend

cp package.json build/package.json
cp plugin.json build/plugin.json
cp main.py build/main.py
cp README.md build/README.md
cp LICENSE build/LICENSE