use crate::Part;

const INPUT: &str = include_str!("../../data/year2023/day01/input.txt");

const DIGIT_NAMES: [&[u8]; 9] = [
    b"one", b"two", b"three", b"four", b"five", b"six", b"seven", b"eight", b"nine",
];

pub fn solve(part: Part, input: Option<&str>) -> u32 {
    let eval = match part {
        Part::One => line_value1,
        Part::Two => line_value2,
    };
    input.unwrap_or(INPUT).lines().map(eval).sum()
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
        if line[i].is_ascii_digit() {
            return line[i] - b'0';
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
    fn test_part1() {
        let example = "\
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
";
        let values: Vec<_> = example.lines().map(line_value1).collect();
        assert_eq!(vec![12, 38, 15, 77], values);
        assert_eq!(142, solve(Part::One, Some(example)));
    }

    #[test]
    fn test_part2() {
        let example = "\
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
";
        let values: Vec<_> = example.lines().map(line_value2).collect();
        assert_eq!(vec![29, 83, 13, 24, 42, 14, 76], values);
        assert_eq!(281, solve(Part::Two, Some(example)));
    }
}
