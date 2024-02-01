use crate::util::grid::Grid;

const INPUT: &str = include_str!("../../data/year2023/day03/input.txt");

#[derive(PartialEq, Debug)]
enum Cell {
    Digit(u32),
    Gear(u32, u32),
    Symbol,
    Empty,
}

impl From<u8> for Cell {
    fn from(byte: u8) -> Self {
        match byte {
            b'0'..=b'9' => Self::Digit((byte - b'0') as u32),
            b'*' => Self::Gear(0, 1),
            b'.' => Self::Empty,
            _ => Self::Symbol,
        }
    }
}

type MyGrid = Grid<Cell>;

pub fn solve() -> (u32, u32) {
    solve_input(INPUT)
}

pub fn solve_input(input: &str) -> (u32, u32) {
    let mut grid = MyGrid::parse(input, b'.');
    let mut part1 = 0;
    for i in 1..=grid.m - 1 {
        let mut parsing = false;
        let mut num = 0;
        let mut j_start = 0;
        for j in 1..=grid.n - 1 {
            match grid[i][j] {
                Cell::Digit(d) => {
                    num = num * 10 + d;
                    if !parsing {
                        parsing = true;
                        j_start = j;
                    }
                }
                _ => {
                    if parsing {
                        //println!("line {i}: found {num} from {j_start} to {j}");
                        if check_number(&mut grid, i, j_start, j - 1, num) {
                            part1 += num;
                        }
                        parsing = false;
                        num = 0;
                    }
                }
            }
        }
    }
    let part2 = grid
        .cells
        .iter()
        .map(|c| match c {
            Cell::Gear(count, prod) if *count == 2 => *prod,
            _ => 0,
        })
        .sum();
    (part1, part2)
}

fn check_number(grid: &mut MyGrid, i: usize, j_start: usize, j_end: usize, num: u32) -> bool {
    let mut is_part = false;
    let mut update = |i, j: usize| match grid[i][j] {
        Cell::Gear(count, prod) => {
            is_part = true;
            grid[i][j] = Cell::Gear(count + 1, prod * num);
        }
        Cell::Symbol => {
            is_part = true;
        }
        _ => (),
    };

    for j in j_start - 1..=j_end + 1 {
        update(i - 1, j);
        update(i + 1, j);
    }
    update(i, j_start - 1);
    update(i, j_end + 1);
    is_part
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let example = "\
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
        let mut grid = MyGrid::parse(example, b'.');
        assert!(check_number(&mut grid, 1, 1, 3, 467));
        assert!(!check_number(&mut grid, 1, 6, 8, 114));
        assert!(check_number(&mut grid, 3, 3, 4, 35));
        assert_eq!(Cell::Gear(2, 16345), grid[2][4]);
        assert_eq!((4361, 467835), solve_input(example));
    }
}
