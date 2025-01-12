#!/bin/bash

current="$(gdate +%s)"
target="$(gdate -d '23:00:10' +%s)"
seconds=$(dc <<< "$target $current - 0 + p") || exit

steps=60
while [ "$seconds" -gt "$steps" ]; do
    sleep "$steps"
    #est=$((seconds - steps))
    current="$(gdate +%s)"
    seconds=$(dc <<< "${target:-0} ${current:-0} - 5 + p") || exit
    #difference=$(dc <<< "${est:-0} ${seconds:-0} - p") || exit
done

if [ "$seconds" -lt 0 ]; then
    seconds=0
fi
sleep "$seconds" && date
