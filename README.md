# Advent of Code 2024

Third time's the charm. I attempted Rust in 2021 or so and very quickly realized
that this wasn't a language I could easily pick up. If you already know C++,
Java, C#, or maybe JavaScript and Go then any of those languages are pretty
reachable. Likewise, if you're already comfortable with Julia then Python should
feel pretty easy.

Rust hasn't been like this for me. I read most of the Rust Book and also most
of Rust By Example, both of which require some effort to get through. Still,
I learned the hard way (by naïvely using *Mathematica* cold on AoC years ago)
that it pays to read a book first. Last year, I read *The Go Programming Language*
by Donovan and Kernighan and it was very helpful.

# Daily Themes and Stars

1. `**` programming basics: parsing input, loops, sorting...this is day 1?
2. `**` corner cases, refactoring, premature optimization, tests, $O(8) = O(1)$
3. `**` regular expressions, reading the instructions (example 2 is not the same as example 1)
4. `**` 2D arrays, bounds, regex doesn't work, graph traversal
5. `**` sorting with custom comparators
6. `**` object-oriented programming, order of operations, cycles, grids, why is this so slow?
7. `**` recursion, operator precedence, digit string concatenation, why is this so fast?
8. `**` vectors, procedural programming, [grids](https://docs.rs/grid/latest/grid/), distinct values (sets; deduplication)

# Lessons Learned

* VSCode helps so much when it shows the inferred types.
* Sprinkle `&` on [closure parameters](https://doc.rust-lang.org/rust-by-example/fn/closures/closure_examples/iter_any.html) to make them more comfortable to use.
* Use `&[T]` instead of `Vec<T>` as [function arguments](https://users.rust-lang.org/t/when-does-one-use-slices-as-arguments/89499).
* `count()` might not do what you expect. Did you mean `len()`? You might need to collect the iterator into a collection.
* `filter_map()` makes clever use of optionals to combine filter and map. It composes nicely with `match`.
* Rust regex doesn't support lookahead (`?=`). Might have been useful for day 4.
* `Vec<Vec<char>>` sorta works for 2D strings, but it isn't as clean as you'd like.
* Rust apparently has no try/catch.
There isn't a great way to backtrack from array accesses with yolo bound checks.
I think you could avoid the panic with `get`, but this didn't work with my `Vec<Vec<_>>`.
A hashmap might have been much easier than nested vectors.
* `is_sorted_by` and `sort_by` expect different comparison functions.
`is_sorted_by` operates on booleans, `sort_by` uses `Ordering`.
* Day 6 was difficult for me. I came up with an interesting solution
([inspired by a Reddit comment](https://www.reddit.com/r/adventofcode/comments/1h7vpqi/comment/m0oxavw/))
based on TTLs instead of keeping the path explored. It was a case where compute is faster than memory.
* Someone else on [Reddit helped me with an extra test case](https://www.reddit.com/r/adventofcode/comments/1h81nc0/comment/m0ppjcy/).
* `ilog` on integers is [much faster](https://www.reddit.com/r/adventofcode/comments/1h8l3z5/comment/m0vp3p7/) than casting to and from float types for logarithms.

# References

* [Rust Book](https://doc.rust-lang.org/book/)
* [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
* [A Gentle Introduction To Rust](https://stevedonovan.github.io/rust-gentle-intro/)
* [Back to the Building Blocks: A Path Toward Secure and Measurable Software](https://www.whitehouse.gov/wp-content/uploads/2024/02/Final-ONCD-Technical-Report.pdf)
* [Response to the Office of the National Cyber Director's Request for Information on Open Source Software Security](https://www.regulations.gov/comment/ONCD-2023-0002-0045)
