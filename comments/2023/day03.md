# Year 2023, day 02

Grids appear quite often in AoC. [Day 3](https://adventofcode.com/2023/day/3) was an opportunity to start developing grid-related code. The `Grid` struct is quite simple for the moment. Generic cells are stored in a 1D vector. For the moment the grid provides associated function for parsing the input and index implementation. To manage the border cells in Java, I usually use a `Grid` class with `boolean isInside(int i, int j)` method. Since Rust only allows to index vectors with `usize`, such an approach would involve a lot of casting. That is why I'm trying something different here: border cells around the original grid.

The problem itself is not very difficult. To make Part 2 easier, I stock two values in each `Gear` cell: the count of the numbers around and their product. I update them when I discover a number and scan around it to decide if it is part number or not. This is done in the `check_number()` function.
