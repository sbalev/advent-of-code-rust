const INPUT: &str = include_str!("../../data/year2023/day05/input.txt");

#[derive(PartialEq, Debug)]
struct RangeMap {
    start: i64,
    end: i64,
    shift: i64,
}

impl RangeMap {
    fn parse(line: &str) -> Self {
        let mut it = line.split_ascii_whitespace();
        let mut next_i64 = || it.next().unwrap().parse().unwrap();
        let (d_start, start, length) = (next_i64(), next_i64(), next_i64());
        Self {
            start,
            end: start + length - 1,
            shift: d_start - start,
        }
    }
}

#[derive(Debug)]
struct Converter {
    ranges: Vec<RangeMap>,
}

impl Converter {
    fn build<'a, I: Iterator<Item = &'a str>>(iter: &mut I) -> Self {
        let mut ranges: Vec<_> = iter
            .take_while(|l| *l != "")
            .map(|l| RangeMap::parse(l))
            .collect();
        ranges.sort_unstable_by_key(|r| r.start);

        // fill the gaps
        let mut i = 1;
        while i < ranges.len() {
            let start = ranges[i - 1].end + 1;
            let end = ranges[i].start - 1;
            if start <= end {
                ranges.insert(
                    i,
                    RangeMap {
                        start,
                        end,
                        shift: 0,
                    },
                );
            }
            i += 1;
        }

        // sentinels
        ranges.insert(
            0,
            RangeMap {
                start: i64::MIN,
                end: ranges[0].start - 1,
                shift: 0,
            },
        );
        ranges.push(RangeMap {
            start: ranges[ranges.len() - 1].end + 1,
            end: i64::MAX,
            shift: 0,
        });
        Self { ranges }
    }

    fn search(&self, val: i64) -> usize {
        match self.ranges.binary_search_by_key(&val, |r| r.start) {
            Ok(i) => i,
            Err(i) => i - 1,
        }
    }

    fn convert(&self, src: i64) -> i64 {
        src + self.ranges[self.search(src)].shift
    }

    fn convert_interval(&self, src: &(i64, i64), result: &mut Vec<(i64, i64)>) {
        let mut start = src.0;
        let end = src.1;
        let mut i = self.search(start);
        while start <= end {
            let r = &self.ranges[i];
            let m = end.min(r.end);
            result.push((start + r.shift, m + r.shift));
            start = m + 1;
            i += 1;
        }
    }
}

fn conv_chain(converters: &[Converter], seed: i64) -> i64 {
    let mut x = seed;
    for conv in converters {
        x = conv.convert(x);
    }
    x
}

fn parse(input: &str) -> (Vec<i64>, Vec<Converter>) {
    let mut iter = input.lines();
    let seeds = iter
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .skip(1)
        .map(|s| s.parse::<i64>().unwrap())
        .collect();

    iter.next();
    let mut converters = Vec::new();
    while let Some(_) = iter.next() {
        converters.push(Converter::build(&mut iter))
    }

    (seeds, converters)
}

pub fn solve() -> (i64, i64) {
    solve_input(INPUT)
}

pub fn solve_input(input: &str) -> (i64, i64) {
    let (seeds, converters) = parse(input);

    let part1 = seeds
        .iter()
        .map(|s| conv_chain(&converters, *s))
        .min()
        .unwrap();

    let mut src: Vec<_> = seeds
        .iter()
        .array_chunks::<2>()
        .map(|[start, length]| (*start, start + length - 1))
        .collect();

    let mut dest = Vec::new();
    for conv in converters {
        for s in &src {
            conv.convert_interval(&s, &mut dest);
        }
        (src, dest) = (dest, src);
        dest.clear();
    }

    let part2 = src.iter().map(|(a, _)| *a).min().unwrap();

    (part1, part2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let example = "\
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
";

        let (seeds, converters) = parse(example);

        assert_eq!(
            RangeMap {
                start: 50,
                end: 97,
                shift: 2
            },
            converters[0].ranges[1]
        );

        let mut x = seeds[0];
        let mut v = vec![x];
        for conv in &converters {
            x = conv.convert(x);
            v.push(x);
        }
        assert_eq!(vec![79, 81, 81, 81, 74, 78, 78, 82], v);

        let v: Vec<_> = seeds.iter().map(|s| conv_chain(&converters, *s)).collect();
        assert_eq!(vec![82, 43, 86, 35], v);

        assert_eq!((35, 46), solve_input(example));
    }
}
