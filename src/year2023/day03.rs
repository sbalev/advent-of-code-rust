use crate::{util::grid::Grid, Part};

const INPUT: &str = include_str!("../../data/year2023/day03/input.txt");

struct Cell {
    byte: u8,
    part_count: u32,
    part_prod: u32,
}

impl From<u8> for Cell {
    fn from(byte: u8) -> Self {
        Cell {
            byte,
            part_count: 0,
            part_prod: 1,
        }
    }
}

type MyGrid = Grid<Cell>;

pub fn solve(part: Part, input: Option<&str>) -> u32 {
    let mut grid = MyGrid::parse(input.unwrap_or(INPUT));
    let mut sum = 0;
    for i in 1..=grid.m {
        let mut parsing = false;
        let mut num: u32 = 0;
        let mut j_start = 1;
        for j in 1..=grid.n {
            let b = grid.get(i, j).byte;
            if parsing {
                if b.is_ascii_digit() {
                    num = 10 * num + ((b - b'0') as u32);
                } else {
                    // found number
                    if is_part(&mut grid, i, j_start, j - 1, num) {
                        sum += num;
                    }
                    // println!("line {}: found {} form {} to {}", i, num, j_start, j - 1);
                    parsing = false;
                }
            } else if b.is_ascii_digit() {
                // start a new number
                parsing = true;
                num = (b - b'0') as u32;
                j_start = j;
            }
        }
        if parsing {
            // found number at the end of a line
            let m = grid.m;
            if is_part(&mut grid, i, j_start, m, num) {
                sum += num;
            }
            // println!("line {}: found {} form {} to {}", i, num, j_start, grid.m);
        }
    }
    match part {
        Part::One => sum,
        Part::Two => grid
            .cells
            .iter()
            .filter(|cell| cell.byte == b'*' && cell.part_count == 2)
            .map(|c| c.part_prod)
            .sum(),
    }
}

fn is_part(grid: &mut MyGrid, line: usize, j_start: usize, j_end: usize, num: u32) -> bool {
    let mut symbol_count = 0;
    for i in line - 1..=line + 1 {
        for j in j_start - 1..=j_end + 1 {
            if grid.is_inside(i, j) {
                let cell = grid.get_mut(i, j);
                if cell.byte != b'.' && !cell.byte.is_ascii_digit() {
                    symbol_count += 1;
                    // side effect for part 2
                    if cell.byte == b'*' {
                        cell.part_count += 1;
                        cell.part_prod *= num;
                    }
                }
            }
        }
    }
    symbol_count > 0
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
";
    #[test]
    fn test_part1() {
        assert_eq!(4361, solve(Part::One, Some(EXAMPLE)));
    }

    #[test]
    fn test_part2() {
        assert_eq!(467835, solve(Part::Two, Some(EXAMPLE)));
    }
}
