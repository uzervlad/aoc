# aoc

This repository contains all of my solutions for Advent of Code

## Years

- [2023](./aoc2023)
- [2024](./aoc2024)
- [2025](./aoc2025)

## Alternatives

Pretty much all solutions use Rust.

Except the very quirky ones, which can be found [here](/alt/)

## Usage

1. Add the input files to the corresponding `inputs/<year>` directory
    - Must be in the format `<day>.txt`
    - Can be suffixed, e.g. `<day>.<suffix>.txt`,
2. Run `cargo run <year> <day> [suffix]`
    - Optionally add `--bench` flag to benchmark the solution over multiple runs
        - `--runs <N>` can be used to specify the number of runs