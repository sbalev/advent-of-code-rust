# Year 2023, day 02

[Day 2](https://adventofcode.com/2023/day/1) is another parsing task. Parsing gets simpler if we realize that we don't need to group together the different draws. What matters is just the maximum number of cubes of each color in a game. So we can just split each line on the whitespaces, skip the first two items and process the remaining items as pairs (value, color).

The control flow of the functions `is_possible()` (Part 1) and `power()` (Part 2) is very similar but the first one can return early before reaching the end of the game. I couldn't find an elegant way to unify them without making the code more complex and less readable.
