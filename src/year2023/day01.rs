const INPUT: &str = include_str!("../../data/year2023/day01/input.txt");

pub fn part1(input: Option<&str>) -> u32 {
    input.unwrap_or(INPUT).lines().map(line_value).sum()
}

pub fn part2(input: Option<&str>) -> u32 {
    0
}

fn line_value(line: &str) -> u32 {
    let first = line.bytes().find(u8::is_ascii_digit).unwrap() - b'0';
    let last = line.bytes().rfind(u8::is_ascii_digit).unwrap() - b'0';
    (10 * first + last) as u32
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
        let values: Vec<_> = example.lines().map(line_value).collect();
        assert_eq!(vec![12, 38, 15, 77], values);
        assert_eq!(142, part1(Some(example)));
    }
}
