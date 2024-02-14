const INPUT: &str = include_str!("../../data/year2023/day07/input.txt");

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Hand {
    value: u8,
    strengths: [u8; 5],
    bid: u32,
}

impl Hand {
    fn parse(desc: &str) -> Self {
        let mut strengths = [0; 5];
        for i in 0..5 {
            strengths[i] = match desc.as_bytes()[i] {
                b'T' => 10,
                b'J' => 11,
                b'Q' => 12,
                b'K' => 13,
                b'A' => 14,
                digit => digit - b'0',
            }
        }
        let bid = desc.split_at(6).1.parse().unwrap();

        let mut counts = [0u8; 15];
        for s in strengths {
            counts[s as usize] += 1;
        }
        counts[2..].sort_unstable();
        let value = 10 * counts[14] + counts[13];

        Self {
            value,
            strengths,
            bid,
        }
    }

    fn consider_jokers(&mut self) {
        self.strengths
            .iter_mut()
            .filter(|s| **s == 11)
            .for_each(|s| *s = 1);

        let mut counts = [0u8; 15];
        for s in self.strengths {
            counts[s as usize] += 1;
        }
        counts[2..].sort_unstable();
        self.value = 10 * (counts[14] + counts[1]) + counts[13];
    }
}

fn total_winnings(hands: &[Hand]) -> u32 {
    hands
        .iter()
        .enumerate()
        .map(|(r, h)| ((r + 1) as u32) * h.bid)
        .sum()
}

pub fn solve() -> (u32, u32) {
    solve_input(INPUT)
}

pub fn solve_input(input: &str) -> (u32, u32) {
    let mut hands: Vec<_> = input.lines().map(Hand::parse).collect();
    hands.sort_unstable();
    let part1 = total_winnings(&hands);

    hands.iter_mut().for_each(|h| h.consider_jokers());
    hands.sort_unstable();
    let part2 = total_winnings(&hands);

    (part1, part2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let example = "\
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
";
        assert_eq!((6440, 5905), solve_input(example));
    }
}
