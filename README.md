My solutions for the "advent of code" challenge of 2021.
===

[Advent of code 2021](https://adventofcode.com/2021)

[![Unit tests](https://github.com/pierrechevalier83/advent_of_code_2021/actions/workflows/rust.yml/badge.svg)](https://github.com/pierrechevalier83/advent_of_code_2021/actions/workflows/rust.yml)

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

## Day 4

```
Day4 - Part1/(default)  time:   [7.0234 us 7.0278 us 7.0348 us]
Found 9 outliers among 100 measurements (9.00%)
  5 (5.00%) high mild
  4 (4.00%) high severe

Day4 - Part2/(default)  time:   [18.591 us 18.603 us 18.616 us]
Found 6 outliers among 100 measurements (6.00%)
  5 (5.00%) high mild
  1 (1.00%) high severe
```

## Day 5

```
Day5 - Part1/(default)  time:   [1.8155 ms 1.8168 ms 1.8182 ms]
Found 4 outliers among 100 measurements (4.00%)
  2 (2.00%) high mild
  2 (2.00%) high severe

Day5 - Part2/(default)  time:   [2.5128 ms 2.5204 ms 2.5286 ms]
Found 11 outliers among 100 measurements (11.00%)
  7 (7.00%) high mild
  4 (4.00%) high severe
  ```

## Day 6

```
Day6 - Part1/(default)  time:   [468.32 ns 469.11 ns 470.13 ns]
Found 6 outliers among 100 measurements (6.00%)
  2 (2.00%) high mild
  4 (4.00%) high severe

Day6 - Part2/(default)  time:   [1.4850 us 1.4852 us 1.4854 us]
Found 12 outliers among 100 measurements (12.00%)
  5 (5.00%) high mild
  7 (7.00%) high severe
```

## Day 7

```
Day7 - Part1/(default)  time:   [11.867 us 11.881 us 11.897 us]
Found 6 outliers among 100 measurements (6.00%)
  5 (5.00%) high mild
  1 (1.00%) high severe

Day7 - Part2/(default)  time:   [19.746 us 19.751 us 19.756 us]
Found 2 outliers among 100 measurements (2.00%)
  1 (1.00%) high mild
  1 (1.00%) high severe
  ```

## Day 8

```
Day8 - Part1/(default)  time:   [1.1012 us 1.1017 us 1.1022 us]
Found 6 outliers among 100 measurements (6.00%)
  1 (1.00%) low mild
  3 (3.00%) high mild
  2 (2.00%) high severe

Day8 - Part2/(default)  time:   [18.148 us 18.204 us 18.262 us]
```

## Day 9

```
Day9 - Part1/(default)  time:   [15.850 us 15.856 us 15.862 us]
Found 7 outliers among 100 measurements (7.00%)
  1 (1.00%) low mild
  3 (3.00%) high mild
  3 (3.00%) high severe

Day9 - Part2/(default)  time:   [2.2132 ms 2.2168 ms 2.2212 ms]
Found 14 outliers among 100 measurements (14.00%)
  4 (4.00%) low mild
  2 (2.00%) high mild
  8 (8.00%) high severe
```

## Day 10

```
Day10 - Part1/(default) time:   [34.503 us 34.527 us 34.549 us]
Found 2 outliers among 100 measurements (2.00%)
  1 (1.00%) high mild
  1 (1.00%) high severe

Day10 - Part2/(default) time:   [33.861 us 33.897 us 33.938 us]
```

## Day 11

```
Day11 - Part1/Buckets   time:   [347.69 us 347.96 us 348.25 us]
Found 3 outliers among 100 measurements (3.00%)
  3 (3.00%) high mild
Day11 - Part1/Grid      time:   [235.95 us 236.29 us 236.66 us]
Found 6 outliers among 100 measurements (6.00%)
  1 (1.00%) low mild
  2 (2.00%) high mild
  3 (3.00%) high severe

Day11 - Part2/Buckets   time:   [1.1559 ms 1.1572 ms 1.1588 ms]
Found 2 outliers among 100 measurements (2.00%)
  1 (1.00%) high mild
  1 (1.00%) high severe
Day11 - Part2/Grid      time:   [803.00 us 804.98 us 807.16 us]
Found 12 outliers among 100 measurements (12.00%)
  3 (3.00%) low mild
  9 (9.00%) high severe
```

## Day 12

```
Day12 - Part1/(default) time:   [83.275 us 83.317 us 83.367 us]
Found 4 outliers among 100 measurements (4.00%)
  1 (1.00%) high mild
  3 (3.00%) high severe

Day12 - Part2/(default) time:   [2.4691 ms 2.4880 ms 2.5070 ms]
  ```

## Day 13

```
Day13 - Part1/(default) time:   [18.714 us 18.796 us 18.892 us]
Found 14 outliers among 100 measurements (14.00%)
  6 (6.00%) high mild
  8 (8.00%) high severe

Day13 - Part2/(default) time:   [124.91 us 125.06 us 125.23 us]
Found 8 outliers among 100 measurements (8.00%)
  5 (5.00%) high mild
  3 (3.00%) high severe
```

## Day 14

```
Day14 - Part1/(default) time:   [2.5670 us 2.5713 us 2.5755 us]
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild

Day14 - Part2/(default) time:   [10.650 us 10.680 us 10.729 us]
Found 5 outliers among 100 measurements (5.00%)
  2 (2.00%) high mild
  3 (3.00%) high severe
```

## Day 15

```
Day15 - Part1/(default) time:   [908.31 us 914.37 us 921.71 us]
Found 8 outliers among 100 measurements (8.00%)
  2 (2.00%) high mild
  6 (6.00%) high severe

Day15 - Part2/(default) time:   [46.162 ms 46.431 ms 46.696 ms]
```

## Day 16

```
Day16 - Part1/(default) time:   [17.425 us 17.434 us 17.443 us]
Found 6 outliers among 100 measurements (6.00%)
  1 (1.00%) low mild
  4 (4.00%) high mild
  1 (1.00%) high severe

Day16 - Part2/(default) time:   [18.828 us 18.839 us 18.850 us]
```

## Day 17

```
Day17 - Part1/(default) time:   [212.33 us 212.81 us 213.33 us]
Found 4 outliers among 100 measurements (4.00%)
  4 (4.00%) high mild

Day17 - Part2/(default) time:   [238.28 us 239.56 us 240.78 us]
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild
```

## Day 19

**Too slow for bench. Reporting result from `cargo aoc -d19` instead**

```
AOC 2021
Day 19 - Part 1 : 365
	generator: 7.28717ms,
	runner: 269.107763ms

Day 19 - Part 2 : 11060
	generator: 4.994979ms,
	runner: 264.255539ms
```

## Day 20

```
Day20 - Part1/(default) time:   [461.96 us 462.22 us 462.53 us]
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild

Day20 - Part2/(default) time:   [23.782 ms 23.810 ms 23.846 ms]
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high mild
```
