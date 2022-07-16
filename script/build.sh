#!/usr/bin/env bash

mkdir -p ./dist

cargo build --release --locked

case "$OSTYPE" in
    darwin*)
        EXECUTABLE=darwin-amd64
    ;;
    *)
        EXECUTABLE=linux-amd64
    ;;
esac

mv target/release/gh-ranking "./dist/$EXECUTABLE"