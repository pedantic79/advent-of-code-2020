#!/bin/bash

NUM=$(printf "%02d" "$1")

if [ -f "src/day$NUM.rs" ]; then
    echo "day$NUM already exists"
    exit 1
fi

cp "src/template.rs" "src/day$NUM.rs"
gsed -i "s/dayN/day$1/" "src/day$NUM.rs"
gsed -i "/Insert before/i pub mod day$NUM;" "src/lib.rs"
gsed -i "/Insert before/i - \[Day $1:\]\(https://adventofcode.com/2024/day/$1\)\n  - \[solution\]\(src/day$NUM.rs\)" README.md
cargo fmt
