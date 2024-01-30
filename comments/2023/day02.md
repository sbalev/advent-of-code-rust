# Year 2023, day 02

[Day 2](https://adventofcode.com/2023/day/1) is another parsing task. Parsing gets simpler if we realize that we don't need to group together the different draws. What matters is just the maximum number of cubes of each color in a game. So we can just split each line on the whitespaces, skip the first two items and process the remaining items as pairs (value, color).

My first solution used a `while let` loop to iterate over the pairs. Then I discovered [`array_chunks()`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.array_chunks) adapter which makes the solution slightly more elegant. However, to use it, one must enable the unstable feature `iter_array_chunks`.