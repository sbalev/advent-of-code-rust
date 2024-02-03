const INPUT: &str = include_str!("../../data/year2023/day04/input.txt");

pub fn solve() -> (u32, u32) {
    solve_input(INPUT)
}

pub fn solve_input(input: &str) -> (u32, u32) {
    let counts: Vec<_> = input.lines().map(winning_count).collect();

    let part1 = counts.iter().map(|n| (1 << n) >> 1).sum();

    let mut copies = vec![1; counts.len()];
    for i in 0..counts.len() {
        for j in 1..=counts[i] {
            copies[i + j] += copies[i];
        }
    }
    let part2 = copies.iter().sum();

    (part1, part2)
}

fn winning_count(card: &str) -> usize {
    let mut it = card.split_ascii_whitespace().skip(2);
    let mut w = vec![false; 100];
    it.by_ref()
        .take_while(|x| x != &"|")
        .map(|x| x.parse::<usize>().unwrap())
        .for_each(|x| w[x] = true);
    it.map(|x| x.parse::<usize>().unwrap())
        .filter(|x| w[*x])
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let example = "\
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let w: Vec<_> = example.lines().map(winning_count).collect();
        assert_eq!(vec![4, 2, 2, 1, 0, 0], w);
        assert_eq!((13, 30), solve_input(example));
    }
}
