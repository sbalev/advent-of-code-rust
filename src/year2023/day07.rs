const INPUT: &str = include_str!("../../data/year2023/day07/input.txt");

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Hand {
    strengths: usize,
    bid: u32,
}

fn hand_value(meta_counts: &[u8]) -> usize {
    match meta_counts {
        [.., 1] => 6,    // five of a kind
        [.., 1, 0] => 5, // four of a kind
        [0, ..] => 4,    // full house
        [2, ..] => 3,    // three of a kind
        [1, ..] => 2,    // two pair
        [3, ..] => 1,    // one pair
        _ => 0,          // high card
    }
}

fn parse(input: &str) -> (Vec<Vec<Hand>>, Vec<Vec<Hand>>) {
    let mut v1 = Vec::with_capacity(7);
    let mut v2 = Vec::with_capacity(7);
    for _ in 0..7 {
        v1.push(Vec::new());
        v2.push(Vec::new());
    }

    for line in input.lines() {
        let bid = line.split_at(6).1.parse().unwrap();
        let mut strengths = 0;
        let mut strengths2 = 0;
        let mut counts = [0; 15];
        for i in 0..5 {
            let mut s = match line.as_bytes()[i] {
                b'T' => 10,
                b'J' => 11,
                b'Q' => 12,
                b'K' => 13,
                b'A' => 14,
                digit => digit - b'0',
            } as usize;
            counts[s] += 1;
            strengths = (strengths << 4) | s;
            if s == 11 {
                s = 1;
            }
            strengths2 = (strengths2 << 4) | s;
        }
        let mut meta_counts = [0; 6];
        for c in &counts[2..] {
            meta_counts[*c] += 1;
        }
        let mut value = hand_value(&meta_counts[1..]);
        v1[value].push(Hand { strengths, bid });

        if counts[11] > 0 {
            meta_counts[counts[11]] -= 1;
            let mut i = 5 - counts[11];
            while meta_counts[i] == 0 {
                i -= 1;
            }
            meta_counts[i] -= 1;
            meta_counts[i + counts[11]] += 1;
            value = hand_value(&meta_counts[1..]);
        }
        v2[value].push(Hand {
            strengths: strengths2,
            bid,
        });
    }

    (v1, v2)
}

fn total_winnings(hands: &[Vec<Hand>]) -> u32 {
    hands
        .iter()
        .flatten()
        .enumerate()
        .map(|(r, h)| (r as u32 + 1) * h.bid)
        .sum()
}

pub fn solve() -> (u32, u32) {
    solve_input(INPUT)
}

pub fn solve_input(input: &str) -> (u32, u32) {
    let (mut v1, mut v2) = parse(input);
    v1.iter_mut()
        .chain(v2.iter_mut())
        .for_each(|v| v.sort_unstable());
    (total_winnings(&v1), total_winnings(&v2))
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
