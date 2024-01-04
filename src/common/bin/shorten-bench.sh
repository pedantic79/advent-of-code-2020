#!/bin/bash

SOURCE=target/aoc/aoc-autobench/benches/aoc_benchmark.rs
NUM=${1:-10}

if [ -e "$SOURCE" ]; then
    gsed -i "s/^criterion_group.*$/criterion_group!\(name = benches; config = Criterion::default\(\).sample_size\($NUM\); targets = aoc_benchmark\);/g" "$SOURCE"
    (cd target/aoc/aoc-autobench && cargo bench)
fi
