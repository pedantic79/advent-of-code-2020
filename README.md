# Advent of Code 2020 - Rust

Requires Rust 1.94 or newer. Run `cargo test --locked --all-features` to test the
solutions and shared utilities.

Install the runner with `cargo install aoc-harness-cli --version 0.6.0 --locked`.
Run these commands from this directory:

```sh
cargo aoc-harness list
cargo aoc-harness run --all-days
cargo aoc-harness run --day 1 --part 2
cargo aoc-harness run --all-days --all-solutions --format json
cargo aoc-harness bench --day 1 --family generator,solver --smoke
cargo aoc-harness bench --all-days --family full-run --sample-size 10
cargo aoc-harness bench --day 1 --save-baseline before
cargo aoc-harness bench --day 1 --baseline before
cargo aoc-harness profile --day 1 --part 2
```

Without a day selection, the harness runs the latest registered day.
`--all-solutions` checks that variants of each part agree. A benchmark smoke
run executes each selected workload once. Use `--family full-run` to time all
selected days together, or `--family end-to-end` to include parsing for each
solution separately.

Inputs are read from `input/2020/dayN.txt`. Initialize the input submodule with
`git submodule update --init input` if you have access, or configure your session
token in `~/.adventofcode.session` and run `cargo aoc-harness input --all`.
Use `--input PATH` with a single day to override its input. The CLI reads the
event year from `aoc_lib!` in `src/lib.rs`.

Regression tests use `aoc_harness::input!(day)` to read local inputs at test
time. Like the runner, it accepts zero-padded filenames as a fallback and strips
trailing newlines. Without private inputs, run
`cargo test --locked --all-features -- --skip '::regression::'`.

Run `just watch-test 08` to rerun a day's library tests on file changes, or pass
a test-name filter such as `just watch-test regression`. `just add-day N` uses
[the package template](templates/day.rs) to generate a zero-padded source file,
register its public module, and add README links. All 25 days are already
registered. Preview generation with
`cargo aoc-harness generate --day N --dry-run`.

<!-- Insert before -->

See:

- [Advent of Code 2020](https://adventofcode.com/2020/)
- [aoc-harness](https://github.com/pedantic79/aoc-harness)
