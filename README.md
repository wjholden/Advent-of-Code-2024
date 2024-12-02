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
2. `**` corner cases, refactoring, premature optimization, tests

# Lessons Learned

* VSCode helps so much when it shows the inferred types.
* Sprinkle `&` on [closure parameters](https://doc.rust-lang.org/rust-by-example/fn/closures/closure_examples/iter_any.html) to make them more comfortable to use.
* Use `&[T]` instead of `Vec<T>` as [function arguments](https://users.rust-lang.org/t/when-does-one-use-slices-as-arguments/89499).
* `count()` might not do what you expect. Did you mean `len()`? You might need to collect the iterator into a collection.
