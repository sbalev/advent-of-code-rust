# Year 2023, day 04

Still no algorithmic difficulties for [day 4](https://adventofcode.com/2023/day/4). However, I struggled with the `Iterator` API before finding how to process the input lines. The [`take_while()`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.take_while) adapter does a good job but it consumes the iterator at it can't be used anymore. The solution is to borrow the iterator using [by_ref()](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.by_ref). So, to process an input line like this one

```text
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
```

I use something like

```rust
let mut it = card.split_ascii_whitespace().skip(2);
it.by_ref().take_while(|x| x != &"|")... // collect the winning numbers
it.filter(...).count() // count how many of the following numbers are winning
```

As low level optimization, I've spend some time to benchmark different data structures to store the winning numbers. In my instance, there only 10 winning numbers and 25 lookups per card. The results are not very significant for such small numbers, but here thy are.

In the first version, I store the winning numbers in a `Vec<&str>` without even parsing them:

```rust
fn winning_count(card: &str) -> usize {
    let mut it = card.split_ascii_whitespace().skip(2);
    let w: Vec<_> = it.by_ref().take_while(|x| x != &"|").collect();
    it.filter(|x| w.contains(x)).count()
}
```

With this version, the running time for Part 1 only is about 106 µs. The second version is similar, but uses `HashSet`instead of `Vec`and achieves only 130 µs.

```rust
fn winning_count(card: &str) -> usize {
    let mut it = card.split_ascii_whitespace().skip(2);
    let w: HashSet<_> = it.by_ref().take_while(|x| x != &"|").collect();
    it.filter(|x| w.contains(x)).count()
}
```

Yes, the default hashing algorithm in Rust is [slow](https://doc.rust-lang.org/book/ch08-03-hash-maps.html#hashing-functions). I didn't try to implement or use another one for the moment.

Another approach: the numbers are small (less than 100) so we can use a `Vec<bool>` of size 100 to store the winning numbers. This time we have to parse them.

```rust
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
```

This drastically drops the running time for part 1 to only 35 µs. Going further in this direction, we can use a single `u128` number as a bitmap:

```rust
fn winning_count(card: &str) -> usize {
    let mut it = card.split_ascii_whitespace().skip(2);
    let mut w = 0u128;
    it.by_ref()
        .take_while(|x| x != &"|")
        .map(|x| x.parse::<u8>().unwrap())
        .for_each(|x| w |= 1 << x);
    it.map(|x| x.parse::<u8>().unwrap())
        .filter(|x| w & (1 << x) != 0)
        .count()
}
```

Surprisingly for me, this version is slightly slower than the previous one: 39 µs.
