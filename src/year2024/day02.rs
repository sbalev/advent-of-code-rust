const INPUT: &str = include_str!("../../data/year2024/day02/input.txt");

pub fn solve() -> (u32, u32) {
    solve_input(INPUT)
}

pub fn solve_input(input: &str) -> (u32, u32) {
    let mut part1 = 0;
    let mut part2 = 0;
    for line in input.lines() {
        let report: Vec<_> = line
            .split_ascii_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
        if is_safe(&report) {
            part1 += 1;
        } else if is_safe2(&report) {
            part2 += 1;
        }
    }
    (part1, part1 + part2)
}

fn is_safe(report: &[i32]) -> bool {
    let (d_min, d_max) = if report[0] < report[1] {
        (-3, -1)
    } else {
        (1, 3)
    };
    for i in 0..report.len() - 1 {
        let d = report[i] - report[i + 1];
        if d < d_min || d> d_max {
            return false;
        }
    }
    true
}

fn is_safe2(report: &[i32]) -> bool {
    let mut report2 = report[1..].to_vec();
    for i in 0..report.len() - 1 {
        if is_safe(&report2) {
            return true;
        }
        report2[i] = report[i];
    }
    is_safe(&report2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let example = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";
        assert_eq!((2, 4), solve_input(example));
    }
}
