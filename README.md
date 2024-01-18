# Advent of Code

As a learning project, I will try to solve some of the [AoC](https://adventofcode.com/) puzzles in Rust.

*Disclaimer*: I am Rust beginner, my code will probably be neither idiomatic, nor efficient.

Constraints:

* Do not use of external crates. The goal is to learn Rust and to get familiar with the API of it's standard library. This means for example, no regex.
* Use efficient algorithms and do the best to implement them efficiently in Rust.

## Project structure

For the moment, the code is organized as follows, but this organization could change in the future. The crate has modules `yearYYYY` for each year with submodules `dayDD` for each day. A module `util` with some commonly used utilities will be added in the future. I will try to keep the same structure for each day :

```rust
use crate::Part;

const INPUT: &str = include_str!("../../data/yearYYYY/dayDD/input.txt");

pub fn solve(part: Part, input: Option<&str>) -> u32 {
    let lines = input.unwrap_or(INPUT).lines();
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let example = "";
        assert_eq!(0, solve(Part::One, Some(example)));
    }

    #[test]
    fn test_part2() {
        let example = "";
        assert_eq!(0, solve(Part::Two, Some(example)));
    }
}
```

The input is not read from a file but directly integrated in the binary.

The first parameter of `solve()` is an enum with variants `One` and `Two`. The second parameter is used by the unit tests to pass the small examples. When it is `None`, the real `INPUT` is used instead.

## Benchmarks

Measured by `cargo bench` on Intel(R) Core(TM) i7-1365U.

### 2023

|                                    Day|Part 1 (µs)|Part 2 (µs)|Total (µs)|
|-------------------------------------:|----------:|----------:|---------:|
|[01](comments/2023/day01.md "comment")|         15|         17|        32|
|[02](comments/2023/day02.md "comment")|          6|         10|        16|
