# Advent of Code

As a learning project, I will try to solve some of the [AoC](https://adventofcode.com/) puzzles in Rust. 

*Disclaimer*: I am Rust beginner, my code will probably be neither idiomatic, nor efficient.

## Project structure

For the moment, the code is organized as follows, but this organization could change in the future. The crate has modules `yearYYYY` for each year with submodules `dayDD` for each day. A module `util` with some commonly used utilities will be added in the future. I will try to keep the same structure for each day :

```rust
use crate::Part;

const INPUT: &str = include_str!("../../data/yearYYYY/dayDD/input.txt");

pub fn solve(part: Part, input: Option<&str>) -> ... {
    let lines = input.unwrap_or(INPUT).lines();
    todo!();
}
```

The input is not read from a file but directly integrated in the binary.

The first parameter of `solve()` is an enum with variants `One` and `Two`. The second parameter is used by the unit tests to pass the small examples. When it is `None`, the real `INPUT` is used instead.

## Benchmarks

Measured by `cargo bench` on Intel(R) Core(TM) i7-1365U.

### 2023

|Day|Part 1 (µs)|Part 2 (µs)|Total (µs)|
|--:|----------:|----------:|---------:|
| 01|         15|         17|        32|
