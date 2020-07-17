#!/usr/bin/env bash
# rustup target add x86_64-apple-ios aarch64-apple-ios
cargo lipo --release --targets aarch64-apple-ios,x86_64-apple-ios
lipo -create  ./target/x86_64-apple-ios/release/deps/libscheme.a ./target/aarch64-apple-ios/release/libscheme.a -output ./libscheme.a
cp libscheme.a ../SimpleScheme/SimpleScheme
# cp libscheme.dylib ../SimpleScheme/SimpleScheme
cp scheme.h ../SimpleScheme/SimpleScheme
echo "done"
