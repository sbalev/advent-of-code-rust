const INPUT: &str = include_str!("../../data/year2023/day07/input.txt");

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Hand {
    value: u8,
    strengths: [u8; 5],
    bid: u32,
    value2: u8,
}

fn hand_value(mc: &[u8]) -> u8 {
    match mc {
        [.., 1] => 6,    // five of a kind
        [.., 1, 0] => 5, // four of a kind
        [0, ..] => 4,    // full house
        [2, ..] => 3,    // three of a kind
        [1, ..] => 2,    // two pair
        [3, ..] => 1,    // one pair
        _ => 0,          // high card
    }
}

impl Hand {
    fn parse(desc: &str) -> Self {
        let mut strengths = [0; 5];
        let mut counts = [0; 15];
        for i in 0..5 {
            strengths[i] = match desc.as_bytes()[i] {
                b'T' => 10,
                b'J' => 11,
                b'Q' => 12,
                b'K' => 13,
                b'A' => 14,
                digit => digit - b'0',
            };
            counts[strengths[i] as usize] += 1;
        }
        let bid = desc.split_at(6).1.parse().unwrap();

        let mut meta_counts = [0u8; 6];
        for c in counts {
            meta_counts[c] += 1;
        }
        let value = hand_value(&meta_counts[1..]);

        let value2 = if counts[11] > 0 {
            meta_counts[counts[11]] -= 1;
            let mut i = 4;
            while meta_counts[i] == 0 {
                i -= 1;
            }
            meta_counts[i] -= 1;
            meta_counts[i + counts[11]] += 1;
            hand_value(&meta_counts[1..])
        } else {
            value
        };

        Self {
            value,
            strengths,
            bid,
            value2,
        }
    }

    fn consider_jokers(&mut self) {
        self.strengths
            .iter_mut()
            .filter(|s| **s == 11)
            .for_each(|s| *s = 1);

        self.value = self.value2;
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
