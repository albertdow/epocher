# epocher

## Overview

I happen to use UNIX epoch timestamps a lot. I thought it would be
a nice little CLI. It converts a date into an epoch timestamp.

## Features

- Uses `clap` for command line argument parsing.
- Supports dates in the `YYYY-MM-DD` format.
- Validates input dates, considering also leap days.
- Outputs epoch timestamp in seconds and milliseconds.

## Usage

```bash
cargo run src/main -- --date <YYYY-MM-DD>
```

or equivalently (after building with `cargo build --release`)

```bash
./target/release/epocher -d <YYYY-MM-DD>
```

## Notes

I wanted to have as little dependencies as possible, so decided to only use
`clap` for the command line arguments parsing, and refrain from using `chrono`
for datetime manipulations.
