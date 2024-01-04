#!/bin/bash

if [ -z "$1" ]; then
    echo "$0 [dayNN]"
    exit 1
fi

cargo watch -x "test --lib -- --nocapture  --test-threads 1 $1"

