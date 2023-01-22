#!/bin/bash

rm -rf build
mkdir -p build/bin

npm install

pnpm run build
cp -r dist build/

VERSION=$(cat package.json| jq -r '.version')
SHA=$(git rev-parse --short HEAD)
if [ -n "$PR_NUMBER" ]; then
    echo "Build release for PR $PR_NUMBER"
    VERSION="$VERSION-beta+$PR_NUMBER.sha.$SHA"
    sed -i -e 's/^version = .*/version = "'$VERSION'"/' ./backend/Cargo.toml
    sed -i -e 's/"version": .*/\"version\": "'$VERSION'",/' package.json
    cat ./backend/Cargo.toml
    cat package.json
else 
    echo "Building release"
fi

cargo build --release --manifest-path backend/Cargo.toml
cp ./backend/target/release/controller-tools build/bin/backend

cp package.json build/package.json
cp plugin.json build/plugin.json
cp main.py build/main.py
cp README.md build/README.md
cp LICENSE build/LICENSE

mv build ControllerTools
rm -f controller-tools-*.zip
zip -r controller-tools-$VERSION.zip ControllerTools/*
mv ControllerTools build

if [ -n "$PR_NUMBER" ]; then
    git checkout -- backend/Cargo.toml package.json
fi