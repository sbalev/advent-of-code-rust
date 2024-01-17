#![feature(test)]

extern crate test;

mod benches {
    use test::Bencher;

    #[bench]
    fn bench_year2023_day01_part1(b: &mut Bencher) {
        b.iter(|| aoc::year2023::day01::part1(None));
    }

    #[bench]
    fn bench_year2023_day01_part2(b: &mut Bencher) {
        b.iter(|| aoc::year2023::day01::part2(None));
    }
}
