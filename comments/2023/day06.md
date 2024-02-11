# Year 2023, day 06

[Day 6](https://adventofcode.com/2023/day/6) was the easiest of the year. There are two different ways to compute the number of winning strategies:

* Brute-forcing:

    ```rust
    fn ways(time: u64, distance: u64) -> u32 {
        (1..time).filter(|x| x * (time - x) > distance).count() as u32
    }
    ```

* Solving the equation $x(t - x) = d + 1$ and taking $\lfloor x_2 \rfloor - \lceil x_1 \rceil + 1$:

    ```rust
    fn ways(time: u64, distance: u64) -> u32 {
        let t = time as f64;
        let d = distance as f64;
        let det = (t * t - 4.0 * (d + 1.0)).sqrt();
        (((t + det) / 2.0).floor() - ((t - det) / 2.0).ceil() + 1.0) as u32
    }
    ```

The brute-forcing runs in 14 milliseconds on my input, while solving the equation takes only 200 nanoseconds, which is 70,000 times faster!
