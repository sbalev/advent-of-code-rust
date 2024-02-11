const INPUT: &str = include_str!("../../data/year2023/day06/input.txt");

fn parse_line1(line: &str) -> Vec<u64> {
    line.split_ascii_whitespace()
        .skip(1)
        .map(|v| v.parse().unwrap())
        .collect()
}

fn parse_line2(line: &str) -> u64 {
    line.chars()
        .filter(|c| c.is_ascii_digit())
        .collect::<String>()
        .parse()
        .unwrap()
}

fn parse<T>(input: &str, parse_line: fn(&str) -> T) -> (T, T) {
    let mut it = input.lines();
    (
        parse_line(it.next().unwrap()),
        parse_line(it.next().unwrap()),
    )
}

/*fn ways(time: u64, distance: u64) -> u32 {
    (1..time).filter(|x| x * (time - x) > distance).count() as u32
}*/

fn ways(time: u64, distance: u64) -> u32 {
    let t = time as f64;
    let d = distance as f64;
    let det = (t * t - 4.0 * (d + 1.0)).sqrt();
    (((t + det) / 2.0).floor() - ((t - det) / 2.0).ceil() + 1.0) as u32
}

pub fn solve() -> (u32, u32) {
    solve_input(INPUT)
}

pub fn solve_input(input: &str) -> (u32, u32) {
    let (times, distances) = parse(input, parse_line1);
    let part1 = times
        .iter()
        .zip(distances.iter())
        .map(|(t, d)| ways(*t, *d))
        .product();
    let (time, distance) = parse(input, parse_line2);
    let part2 = ways(time, distance);
    (part1, part2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let example = "\
Time:      7  15   30
Distance:  9  40  200
";
        assert_eq!(4, ways(7, 9));
        assert_eq!(8, ways(15, 40));
        assert_eq!(9, ways(30, 200));
        assert_eq!((71530, 940200), parse(example, parse_line2));
        assert_eq!((288, 71503), solve_input(example));
    }
}
