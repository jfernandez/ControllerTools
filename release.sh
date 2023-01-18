#!/bin/bash

rm -rf build
mkdir -p build/bin

npm run build
cp -r dist build/

cargo build --release --manifest-path backend/Cargo.toml
cp ./backend/target/release/controller-tools build/bin/backend

cp package.json build/package.json
cp plugin.json build/plugin.json
cp main.py build/main.py
cp README.md build/README.md
cp LICENSE build/LICENSE

mv build ControllerTools
VERSION=$(cat package.json| jq -r '.version')
rm -f controller-tools-$VERSION.zip
zip -r controller-tools-$VERSION.zip ControllerTools/*
mv ControllerTools build