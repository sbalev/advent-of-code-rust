# Advent of Code

As a learning project, I will try to solve some of the [AoC](https://adventofcode.com/) puzzles in Rust.

*Disclaimer*: I am Rust beginner, my code will probably be neither idiomatic, nor efficient.

Constraints:

* Do not use of external crates. The goal is to learn Rust and to get familiar with the API of it's standard library. This means for example, no regex.
* Use efficient algorithms and do the best to implement them efficiently in Rust.

## Project structure

For the moment, the code is organized as follows, but this organization could change in the future. The crate has modules `yearYYYY` for each year with submodules `dayDD` for each day. A module `util` contains some commonly used tools. I will try to keep the same structure for each day :

```rust
const INPUT: &str = include_str!("../../data/yearYYYY/dayDD/input.txt");

pub fn solve() -> (u32, u32) {
    solve_input(INPUT)
}

pub fn solve_input(input: &str) -> (u32, u32) {
    (0, 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let example = "";
        assert_eq!(0, solve_input(example));
    }
}
```

The input is not read from a file but directly integrated in the binary.

The `solve_input()` function can be called passing either the small examples for testing or the real input.

## Benchmarks

Measured by `cargo bench` on Intel(R) Core(TM) i7-1365U with `RUSTFLAGS="-C target-cpu=native"`.

### 2023

|                                   Day|Time (µs)|
|-------------------------------------:|--------:|
|[01](comments/2023/day01.md "comment")|       32|
|[02](comments/2023/day02.md "comment")|       10|
|03                                    |       68|
