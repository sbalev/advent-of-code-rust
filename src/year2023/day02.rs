const INPUT: &str = include_str!("../../data/year2023/day02/input.txt");

pub fn solve() -> (u32, u32) {
    solve_input(INPUT)
}

pub fn solve_input(input: &str) -> (u32, u32) {
    let max_rgb: Vec<_> = input.lines().map(max_rgb).collect();
    let part1 = max_rgb
        .iter()
        .enumerate()
        .filter(|(_, (r, g, b))| *r <= 12 && *g <= 13 && *b <= 14)
        .map(|(id, _)| (id + 1) as u32)
        .sum();
    let part2 = max_rgb.iter().map(|(r, g, b)| r * g * b).sum();
    (part1, part2)
}

fn max_rgb(game: &str) -> (u32, u32, u32) {
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
    (r, g, b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let example = "\
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
";
        let values: Vec<_> = example.lines().map(max_rgb).collect();
        assert_eq!(
            vec![(4, 2, 6), (1, 3, 4), (20, 13, 6), (14, 3, 15), (6, 3, 2)],
            values
        );
        assert_eq!((8, 2286), solve_input(example))
    }
}
