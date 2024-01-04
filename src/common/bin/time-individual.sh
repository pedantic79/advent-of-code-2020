#!/bin/bash

cargo test --lib --release -- --test-threads=1 regression --report-time -Zunstable-options
