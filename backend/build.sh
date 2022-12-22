#!/bin/bash

mkdir -p ../bin

# Debug
cross build
cp ./target/x86_64-unknown-linux-gnu/debug/controller-tools ../bin/backend

# Release
#TARGET_CC=x86_64-unknown-linux-gnu-gcc cargo build --release --target x86_64-unknown-linux-gnu
#cp ./target/x86_64-unknown-linux-gnu/release/backend ../bin/backend
