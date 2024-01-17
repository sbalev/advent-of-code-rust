# Year 2023, day 01

[Day 1](https://adventofcode.com/2023/day/1) is essentially a parsing task. The only possible gotcha is that the digit names may overlap as in `twoneight`. Rust has no support for regex in it's standard library, but it is as simple as scanning the lines and searching the first and the last digits (or digit names for part 2).

Well, "simple" is not the right word when it comes to strings. Rust takes UTF-8 very seriously. I struggled with the `&str` API but finally decided to work with byte slices. After all, the input is always ASCII and I also suppose that it's faster. Luckily, [as_bytes()](https://doc.rust-lang.org/std/primitive.str.html#method.as_bytes) and [bytes()](https://doc.rust-lang.org/std/primitive.str.html#method.bytes) give handy access to the underlying bytes of a `&str`.

I struggled a bit with the type of the second parameter of `search_digit()`. At the beginning it was a [`Range`](https://doc.rust-lang.org/std/ops/struct.Range.html) and it worked like a charm with `0..line.len()` but not with `(0..line.len()).rev()` which is not a range but ... an iterator. It took me some time to find the right type bound `impl Iterator<Item = usize>`.

Finally, looking for some low level optimizations, I considered implementing some lookup data structure as a [trie](https://en.wikipedia.org/wiki/Trie). Of course, it would be overkill for only 9 entries. But I came up with something which is kind of a root of a trie. There are only six possible first letters of the digit names : `e`, `f`, `n`, `o`, `s` and `t` which can be encoded by a single `u32` value:

```
.... ..zy xwvu tsrq ponm lkji hgfe dcba
0000 0000 0000 1100 0110 0000 0011 0000
   0    0    0    c    6    0    3    0
```

So

```rust
if 0xc6030u32 >> (d - b'a') == 0 {
    continue;
}
```

prevents from looping on the digit names if the current letter is not among the six possibilities. This dropped the benchmark time of Part 2 from 21 to 17 µs, not a big deal but I think it's a neat hack.
