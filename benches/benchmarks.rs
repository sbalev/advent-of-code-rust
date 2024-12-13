#![feature(test)]

extern crate test;

mod benches {
    use aoc::*;
    use test::Bencher;

    #[bench]
    fn bench_year2023_day01(b: &mut Bencher) {
        b.iter(|| year2023::day01::solve());
    }

    #[bench]
    fn bench_year2023_day02(b: &mut Bencher) {
        b.iter(|| year2023::day02::solve());
    }

    #[bench]
    fn bench_year2023_day03(b: &mut Bencher) {
        b.iter(|| year2023::day03::solve());
    }

    #[bench]
    fn bench_year2023_day04(b: &mut Bencher) {
        b.iter(|| year2023::day04::solve());
    }

    #[bench]
    fn bench_year2023_day05(b: &mut Bencher) {
        b.iter(|| year2023::day05::solve());
    }

   #[bench]
    fn bench_year2023_day06(b: &mut Bencher) {
        b.iter(|| year2023::day06::solve());
    }

   #[bench]
    fn bench_year2023_day07(b: &mut Bencher) {
        b.iter(|| year2023::day07::solve());
    }

    #[bench]
    fn bench_year2024_day01(b: &mut Bencher) {
        b.iter(|| year2024::day01::solve());
    }
}
