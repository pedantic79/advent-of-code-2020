#!/bin/bash

if [ -n "$1" ]; then
    tests=$(printf "day%02d::tests::regression" $1)
else
    tests="regression"
fi

hyperfine --warmup 2 "cargo test --release --quiet --lib -- --test-threads=1 $tests"
