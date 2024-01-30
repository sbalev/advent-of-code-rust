const INPUT: &str = include_str!("../../data/year2023/day01/input.txt");

const DIGIT_NAMES: [&[u8]; 9] = [
    b"one", b"two", b"three", b"four", b"five", b"six", b"seven", b"eight", b"nine",
];

pub fn solve() -> (u32, u32) {
    solve_input(INPUT)
}

pub fn solve_input(input: &str) -> (u32, u32) {
    let lines: Vec<_> = input.lines().collect();
    let part1 = lines.iter().map(|l| line_value1(l)).sum();
    let part2 = lines.iter().map(|l| line_value2(l)).sum();
    (part1, part2)
}

fn line_value1(line: &str) -> u32 {
    let first = line.bytes().find(u8::is_ascii_digit).unwrap() - b'0';
    let last = line.bytes().rfind(u8::is_ascii_digit).unwrap() - b'0';
    (10 * first + last) as u32
}

fn line_value2(line: &str) -> u32 {
    let line = line.as_bytes();
    let first = search_digit(line, 0..line.len());
    let last = search_digit(line, (0..line.len()).rev());
    (10 * first + last) as u32
}

fn search_digit(line: &[u8], range: impl Iterator<Item = usize>) -> u8 {
    for i in range {
        let d = line[i];
        if d.is_ascii_digit() {
            return d - b'0';
        }

        // this hack saves about 3 µs
        if 0xc6030u32 & (1 << (d - b'a')) == 0 {
            continue;
        }

        for (digit, name) in DIGIT_NAMES.iter().enumerate() {
            if line[i..].starts_with(name) {
                return (digit + 1) as u8;
            }
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let example1 = "\
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
";
        let values: Vec<_> = example1.lines().map(line_value1).collect();
        assert_eq!(vec![12, 38, 15, 77], values);

        let example2 = "\
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
";
        let values: Vec<_> = example2.lines().map(line_value2).collect();
        assert_eq!(vec![29, 83, 13, 24, 42, 14, 76], values);
    }
}
