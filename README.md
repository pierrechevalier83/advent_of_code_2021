My solutions for the "advent of code" challenge of 2021.
===

[Advent of code 2021](https://adventofcode.com/2021)

This crate uses `cargo-aoc` for automating the boilerplate.

To install it, run
```
cargo install cargo-aoc
```

For each challenge, at least 2 unit tests exist asserting that parts 1 and 2 are correct (after manually verifying the answers on the website). These serve as regression tests in case I touch up some previous days (for instance, to reduce duplication with later days)

# Running existing solutions

To run the current day, use
```
cargo aoc
```
To benchmark the current day, use
```
cargo aoc bench
```

To run a specific day (e.g day 1), use
```
cargo aoc -d1
```
To run a specific day and part (e.g day 1, part 2), use
```
cargo aoc -d1 -p2
```

To run all solutions, use
```
cargo run --release
```
To run all unit tests, use
```
cargo test --release
```

# Preparing a new solution
To download the input for today, run
```
cargo aoc input
```

To download the input for a previous day X, run
```
cargo aoc input -dX
```

Code the generator to parse this day's input and the solutions to each part in `src/dayX.rs`.
Make your solution visible at the top level by adding `pub mod dayX` in `src/lib.rs`.

# Updating the session id 
If the session id expire, log in to the advent of code website, and obtain the cookie id (In Chrome: Shift+F9, Cookies tab, and copy the "Value" for the "session" field).
Then run
```
cargo aoc credentials -s <session id>
```

# Performance so far

Benchmarks obtained on my T14 with AMD Ryzen 7 PRO 4750U.

## Day 1

```
Day1 - Part1/(default)  time:   [489.63 ns 491.12 ns 493.62 ns]
Found 3 outliers among 100 measurements (3.00%)
  1 (1.00%) high mild
  2 (2.00%) high severe

Day1 - Part2/Naive      time:   [3.0232 us 3.0333 us 3.0438 us]
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high mild
Day1 - Part2/NoAlloc    time:   [486.22 ns 486.41 ns 486.63 ns]
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild
```

## Day 2

```
Day2 - Part1/Naive      time:   [1.2443 us 1.2464 us 1.2485 us]
Found 17 outliers among 100 measurements (17.00%)
  3 (3.00%) low severe
  13 (13.00%) low mild
  1 (1.00%) high severe
Day2 - Part1/Unordered  time:   [1.1393 us 1.1401 us 1.1409 us]
Found 2 outliers among 100 measurements (2.00%)
  1 (1.00%) high mild
  1 (1.00%) high severe

Day2 - Part2/(default)  time:   [1.3397 us 1.3408 us 1.3422 us]
Found 6 outliers among 100 measurements (6.00%)
  3 (3.00%) high mild
  3 (3.00%) high severe
```

## Day 3

```
Day3 - Part1/(default)  time:   [591.20 ns 591.29 ns 591.36 ns]
Found 6 outliers among 100 measurements (6.00%)
  1 (1.00%) low severe
  5 (5.00%) high mild

Day3 - Part2/(default)  time:   [16.234 us 16.267 us 16.306 us]
Found 18 outliers among 100 measurements (18.00%)
  4 (4.00%) low severe
  14 (14.00%) low mild
```
