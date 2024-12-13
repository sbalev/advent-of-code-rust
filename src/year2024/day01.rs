const INPUT: &str = include_str!("../../data/year2024/day01/input.txt");

fn parse(input: &str) -> (Vec<u32>, Vec<u32>) {
    input
        .split_ascii_whitespace()
        .array_chunks()
        .map(|[l, r]| (l.parse::<u32>().unwrap(), r.parse::<u32>().unwrap()))
        .unzip()
}

pub fn solve() -> (u32, u32) {
    solve_input(INPUT)
}

pub fn solve_input(input: &str) -> (u32, u32) {
    let (mut left, mut right) = parse(input);
    left.sort_unstable();
    right.sort_unstable();

    let part1 = left
        .iter()
        .zip(right.iter())
        .map(|(l, r)| l.abs_diff(*r))
        .sum();

    let mut part2 = 0;
    let mut prec = left[0] - 1;
    let mut times = 0;
    let mut j = 0;
    for n in left {
        if n != prec {
            while j < right.len() && right[j] < n {
                j += 1;
            }
            times = 0;
            while j < right.len() && right[j] == n {
                times += 1;
                j += 1;
            }
        }
        prec = n;
        part2 += n * times;
    }

    (part1, part2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let example = "\
3   4
4   3
2   5
1   3
3   9
3   3
";
        assert_eq!((11, 31), solve_input(example));
    }
}
