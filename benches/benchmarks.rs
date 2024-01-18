#![feature(test)]

extern crate test;

mod benches {
    use aoc::*;
    use test::Bencher;

    #[bench]
    fn bench_year2023_day01_part1(b: &mut Bencher) {
        b.iter(|| year2023::day01::solve(Part::One, None));
    }

    #[bench]
    fn bench_year2023_day01_part2(b: &mut Bencher) {
        b.iter(|| year2023::day01::solve(Part::Two, None));
    }

    #[bench]
    fn bench_year2023_day02_part1(b: &mut Bencher) {
        b.iter(|| year2023::day02::solve(Part::One, None));
    }

    #[bench]
    fn bench_year2023_day02_part2(b: &mut Bencher) {
        b.iter(|| year2023::day02::solve(Part::Two, None));
    }
}
