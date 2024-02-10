# Year 2023, day 05

Part 1 of [day 5](https://adventofcode.com/2023/day/5) is quite straightforward. To solve quickly part 2, I took care to transform each `foo-to-bar map` in a sorted set of intervals that cover `[-∞..∞]`. in this way, converting a whole interval `[a..b]` becomes easy: binary-search for `a` and then go forward and split until reaching `b`. For example,

```text
             a                b
            [4..6][7..10][11..23]
[-∞..-1][0......6][7..10][11.......52][53..60][61..∞]
```

This is implemented in `convert_interval()`.

I didn't want to collect the input lines in a `Vec` before parsing them, so my parsing is probably more complicated than necessary.

Rust lessons learned :

* use `sort_unstable...` instead of `sort...` when appropriate (most of the time). It's faster. I didn't observe any measurable difference when sorting so few items (44 max in my input) but it's written in the [doc](https://doc.rust-lang.org/std/primitive.slice.html#method.sort).
* Passing an `Iterator` to a function is not trivial because iterators are not `Sized`. Solution: use generics. Hence this complicated method signature:

    ```rust
    fn build<'a, I: Iterator<Item = &'a str>>(iter: &mut I) -> Self { ... }
    ```
