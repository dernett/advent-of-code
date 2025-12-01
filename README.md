# Advent of Code solutions in Rust

This repository contains my solutions to Advent of Code problems while trying
to learn Rust.

## Layout

The project is structured as a workspace with each year gettings its own crate.
Individual solutions are placed in `src/bin` and dependencies are shared
amongst crates.

## Usage

From the top-level directory, run the following commands to build, run, and
test:

* `cargo build -p event-$YEAR --bin day$DAY`

* `cargo run -p event-$YEAR --bin day$DAY`

* `cargo test -p event-$YEAR --bin day$DAY`

By default, these will all be in debug mode. Pass the `--release` flag for
release mode. Build outputs will be placed in `target` under the directory
for the corresponding mode.

## Development

* `scripts/boilerplate` contains a script that will generate a solution
  template and corresponding input file. It takes the year and day to
  generate as input.

