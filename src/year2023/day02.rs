use crate::Part;

const INPUT: &str = include_str!("../../data/year2023/day02/input.txt");

pub fn solve(part: Part, input: Option<&str>) -> u32 {
    let lines = input.unwrap_or(INPUT).lines();
    match part {
        Part::One => lines
            .enumerate()
            .filter(|(_, game)| is_possible(game))
            .map(|(id, _)| (id + 1) as u32)
            .sum(),
        Part::Two => lines.map(power).sum(),
    }
}

/// Checks if a game is possible with 12 red, 13 green and 14 blue cubes
fn is_possible(game: &str) -> bool {
    let mut it = game.split_ascii_whitespace().skip(2);
    while let Some(val) = it.next() {
        let val: u32 = val.parse().unwrap();
        match it.next().unwrap().as_bytes()[0] {
            b'r' if val > 12 => return false,
            b'g' if val > 13 => return false,
            b'b' if val > 14 => return false,
            _ => (),
        }
    }
    true
}

/// Computes the "power" of a game: max(r) * max(g) * max(b)
fn power(game: &str) -> u32 {
    let (mut r, mut g, mut b) = (0, 0, 0);
    let mut it = game.split_ascii_whitespace().skip(2);
    while let Some(val) = it.next() {
        let val: u32 = val.parse().unwrap();
        match it.next().unwrap().as_bytes()[0] {
            b'r' => r = r.max(val),
            b'g' => g = g.max(val),
            b'b' => b = b.max(val),
            _ => (),
        }
    }
    r * g * b
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
";

    #[test]
    fn test_part1() {
        let values: Vec<_> = EXAMPLE.lines().map(is_possible).collect();
        assert_eq!(vec![true, true, false, false, true], values);
        assert_eq!(8, solve(Part::One, Some(EXAMPLE)));
    }

    #[test]
    fn test_part2() {
        let values: Vec<_> = EXAMPLE.lines().map(power).collect();
        assert_eq!(vec![48, 12, 1560, 630, 36], values);
        assert_eq!(2286, solve(Part::Two, Some(EXAMPLE)));
    }
}
