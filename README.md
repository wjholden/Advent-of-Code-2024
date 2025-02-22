# Advent of Code 2024

Rust!

# Daily Themes and Stars

1. `**` programming basics: parsing input, loops, sorting...this is day 1?
2. `**` corner cases, refactoring, premature optimization, tests, $O(8) = O(1)$
3. `**` regular expressions, reading the instructions (example 2 is not the same as example 1)
4. `**` 2D arrays, bounds, regex doesn't work, graph traversal
5. `**` sorting with custom comparators
6. `**` object-oriented programming, order of operations, cycles, grids, why is this so slow?
7. `**` recursion, operator precedence, digit string concatenation, why is this so fast?
8. `**` vectors, procedural programming, [grids](https://docs.rs/grid/latest/grid/), distinct values (sets; deduplication)
9. `**` disk fragmentation, arrays (trickier-than-it-looks, annoying procedural programs, puzzle could probably be solved faster than actually doing it...)
10. `**` searching (I used BFS), grids, path-finding
11. `**` dynamic programming, recursion, automata
12. `**` more grids, divide-and-conquer was not the solution, graph searches
13. `**` linear algebra (`Ax=b`), greedy algorithms, integer division loses information
14. `**` corner case (well, "center case" since the middle doesn't count), constants, modulus operator, even more grids, find the Christmas tree
15. `**` still more grids, refactoring, procedural programming/mutable state, so many cases, order of operations, queues
16. `**` A*, pathfinding, complex arithmetic
17. `**` assembly, backpropagation, too big for [Goal Seek](https://www.reddit.com/r/adventofcode/comments/1hga8be/2024_day_17_part_2_excel_goal_seek_had_no_chance/)
18. `**` pathfinding, bisection
19. `**` tries, dynamic programming/memoization
20. `**` cost analysis (this problem is smaller than it looks), abstraction, even more index trickiness
21. `**` robotics, indirection, nested loops, dynamic programming/memoization, **hard problems**
22. `**` arithmetic, complicated instructions, subsequences, pseudo-random number generators (PRNG), MapReduce
23. `**` maximal clique problem, [matrix-based solution](https://www.reddit.com/r/adventofcode/comments/1hkgj5b/comment/m3g3jn2/), coping with NP-hard problems
24. `**` logic gates
25. `**` combinations

# Lessons Learned

* VSCode helps so much when it shows the inferred types. This feature is called
[inlay hints](https://code.visualstudio.com/docs/languages/rust#_inlay-hints).
* On the other hand, RustRover's Copilot-like predictions would take some getting
used to. I found myself accepting predictions that looked right, but later turned
out to have subtle mistakes that were tricky to debug. I know lots of people feel
more productive iwth Copilot, but for now I need to learn the language for myself.
* Consider using `into_iter` when it's OK to consume the iterator or decorate
[closure parameters](https://doc.rust-lang.org/rust-by-example/fn/closures/closure_examples/iter_any.html)
with `&` to make them a little more comfortable.
* Use `&[T]` instead of `Vec<T>` as [function arguments](https://users.rust-lang.org/t/when-does-one-use-slices-as-arguments/89499).
* `count()` might not do what you expect. Did you mean `len()`? You might need to
collect the iterator into a collection.
* `filter_map()` makes clever use of optionals to combine filter and map. It
composes nicely with `match`.
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
based on TTLs instead of keeping the path explored. It was a case where compute
is faster than memory.
* My Day 6 solution is among my slowest programs. I wonder if a grid would have
been better than the hashmap.
* Someone else on [Reddit helped me with an extra test case](https://www.reddit.com/r/adventofcode/comments/1h81nc0/comment/m0ppjcy/).
* `ilog` on integers is [much faster](https://www.reddit.com/r/adventofcode/comments/1h8l3z5/comment/m0vp3p7/) than 
casting to and from float types for logarithms.
* https://stackoverflow.com/questions/40006219/why-is-it-discouraged-to-accept-a-reference-string-vec-or-box-as-a-function
* https://stackoverflow.com/questions/30633177/implement-fmtdisplay-for-vect
* Expect compiler errors or runtime crashes on integer overflow. One should
prefer addition to subtraction when comparising distance. z.B, to check if
`a: u8` is one less than `b: u8`, use `a + 1 == b` instead of `b - a == 1`.
* Day 11 was a tricky dymanic programming problem. Two tricks: you don't need to
worry about the stone order (despite the phrasing of the prompt), and you only 
need to count occurrences of the numbered stones. I had my head wrapped around
a jagged recursive triangle, but you don't need that. This is more like the 
iterative Fibonacci approach with `while i < k { (a, b) = (a + b, b); i += 1 }`.
You actually *can* use trees, but you need to count down to a basis of `depth=1`.
See [[2024 Day 11][Python] MEGA TUTORIAL](https://www.reddit.com/r/adventofcode/comments/1hbnyx1/2024_day_11python_mega_tutorial/).
See also [[2024 Day 11] Every sequence converges to 3947 points (with proof)](https://www.reddit.com/r/adventofcode/comments/1hbtz8w/2024_day_11_every_sequence_converges_to_3947/)
for an interesting study of attractors in this problem.
* `include_str!` can bring in the contents of a file. My tests show that the
performance is about the same as `std::fs::read_to_string`,
but it's nice to ship a binary with no dependent files.
* In day 9 I had some trouble with the borrow checker. Passing a reference to a
mutable struct into a function might not work. Consider the object-oriented
approach.
* Be careful with the Copy and Clone traits with the members of a mutable array.
You might accidentally copy a value without realizing it, then be surprised
that your writes aren't working. A workaround is to just reference the array
members directly.
* Vector comparison (`v1 < v2`) might not quite do what you expect in [nalgebra](https://docs.rs/nalgebra/).
* I might have to check out [ndarray](https://crates.io/crates/ndarray).
* These procedural days (days 6, 8, 9, 12, 15) are difficult. Maybe Rust makes it worse, maybe I'm just not good at them.
* The swaps I did in day 15 probably made this harder than it needed to be.
I had made an early design decision to model the game as a grid (2D array) with contents,
rather than a collection of game objects with coordinates. Had I opted for the objects,
I could have simply incremented their x/y positions as a group instead of this order-dependent swap nightmare.
* [`flat_map`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.flat_map) combines map and flatten.
* The [`@` operator](https://stackoverflow.com/questions/69435734/rust-what-does-the-at-sign-operator-do) binds matched values of a pattern to values.
* Rust's `std::collections` does not have a
[red-black tree](https://docs.oracle.com/javase/8/docs/api/java/util/TreeMap.html), but it does provide a 
[BTree](https://doc.rust-lang.org/std/collections/btree_map/struct.BTreeMap.html).
* I was surprised that BTreeSet produced slightly faster runtimes than HashSet in day 18.
* Day 21 is **very** difficult. I resorted to modeling the button presses manually using JSON.
[Serde](https://serde.rs/) makes it easy to deserialize a JSON string into a string.
I was surprised that it correctly read JSON string elements into the `char` type for my application.
* I tried unsuccessfully to reduce day 24 to A*, much like the 8-piece puzzle problem.
I still think this method could have worked, but ${222 \choose 2}^4$ is a very large search space.
I solved day 24 by rendering the logic gates with GraphViz, learning how an adder works,
and spot-the-difference troubleshooting.
* You can reference named regex capture groups with square brackets (see day 3).
* [Rust is very fast.](https://www.reddit.com/r/adventofcode/comments/1hrb4j6/2024rust_solving_aoc_2024_in_under_1ms_for_real/)
* [Very very fast.](https://www.reddit.com/r/adventofcode/comments/1hlyocd/500_in_less_than_a_second/)
* (Apparently [Julia is also very fast](https://www.reddit.com/r/adventofcode/comments/1i3bk9n/beating_the_rust_community_in_julia/)).

# References

* [CppNorth Keynote: Advent of Code, Behind the Scenes - Eric Wastl](https://www.youtube.com/watch?v=uZ8DcbhojOw)
* [Rust Book](https://doc.rust-lang.org/book/)
* [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
* [A Gentle Introduction To Rust](https://stevedonovan.github.io/rust-gentle-intro/)
* [Back to the Building Blocks: A Path Toward Secure and Measurable Software](https://www.whitehouse.gov/wp-content/uploads/2024/02/Final-ONCD-Technical-Report.pdf)
* [Response to the Office of the National Cyber Director's Request for Information on Open Source Software Security](https://www.regulations.gov/comment/ONCD-2023-0002-0045)
* [One Hundred Thousand Lines of Rust](https://matklad.github.io/2021/09/05/Rust100k.html)
* [[2024 Day 13] An explanation of the mathematics](https://www.reddit.com/r/adventofcode/comments/1hd7irq/2024_day_13_an_explanation_of_the_mathematics/)
* [[2024 Day 14 Part 2] Why have fun with image detection when you can use maths?](https://www.reddit.com/r/adventofcode/comments/1he0asr/2024_day_14_part_2_why_have_fun_with_image/)
* [(fish-shell) Rewrite it in Rust #9512](https://github.com/fish-shell/fish-shell/pull/9512#issuecomment-1410820102)
* [Rustfinity](https://www.rustfinity.com)
* [Rustlings](https://rustlings.cool)

# Libraries
* [grid](https://docs.rs/grid/) for 2D vectors that don't need to be full matrices.
* [nalgebra](https://docs.rs/nalgebra/) for matrices.
* [socket2](https://docs.rs/socket2/) for future networking projects requiring raw sockets.
* [Ratatui](https://ratatui.rs/) for terminal user interfaces (TUI).
* [clap](https://docs.rs/clap/) for command-line argument parsing.
* [hyper](https://hyper.rs/) and [axum](https://github.com/tokio-rs/axum) for making web applications.
* [Handlebars](https://docs.rs/handlebars/), [Tera](https://keats.github.io/tera/), [Maud](https://maud.lambda.xyz/), and [Askama](https://github.com/rinja-rs/askama) for HTML templating.
* [SQLx](https://github.com/launchbadge/sqlx) for accessing databases.
* [Serde](https://serde.rs/) for serializing and deserializing JSON and many other data formats.
* [nom](https://github.com/rust-bakery/nom) for parsing.
* [Bevy](https://bevyengine.org/) for games.
* [pathfinding](https://rfc1149.net/devel/pathfinding.html) for graphs.
* [Rayon](https://docs.rs/rayon/) for parallelism.
