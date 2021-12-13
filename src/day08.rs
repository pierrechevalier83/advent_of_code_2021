use aoc_runner_derive::{aoc, aoc_generator};
use std::str::FromStr;

struct DigitDisplay(u8);

fn char_index(c: char) -> usize {
    c as usize - 'a' as usize
}

impl FromStr for DigitDisplay {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut display: u8 = 0;
        for c in s.chars() {
            display |= 1 << char_index(c);
        }
        Ok(Self(display))
    }
}

impl DigitDisplay {
    fn contains(&self, c: char) -> bool {
        (self.0 & (1 << char_index(c))) != 0
    }
    fn num_segments(&self) -> usize {
        self.0.count_ones() as usize
    }
    fn chars(&self) -> impl Iterator<Item = char> + '_ {
        ('a'..='g').filter(|&c| self.contains(c))
    }
}

struct Input {
    digits: Vec<DigitDisplay>,
    output: Vec<DigitDisplay>,
}

#[aoc_generator(day08)]
fn parse_input(data: &str) -> Vec<Input> {
    data.lines()
        .map(|line| {
            let mut it = line.split(" | ").map(|segment| {
                segment
                    .split(' ')
                    .map(DigitDisplay::from_str)
                    .collect::<Result<Vec<_>, _>>()
                    .unwrap()
            });
            Input {
                digits: it.next().unwrap(),
                output: it.next().unwrap(),
            }
        })
        .collect()
}

#[aoc(day08, part1)]
fn part1(data: &[Input]) -> usize {
    data.iter()
        .flat_map(|line| line.output.iter())
        .filter(|word| {
            let n = word.num_segments();
            // digit one
            n == 2 ||
            // digit 7
            n == 3 ||
            // digit 4
            n == 4 ||
            // digit 8
            n == 7
        })
        .count()
}

/*
letter to n occurences in digits:
a -> 8
b -> 6 // unique
c -> 8
d -> 7
e -> 4 // unique
f -> 9 // unique
g -> 7
*/
fn count_segment_occurrences(input: &[DigitDisplay]) -> [usize; 7] {
    let mut n_occurrences_of_digit = [0; 7];
    for digit in 'a'..='g' {
        for display in input {
            if display.contains(digit) {
                n_occurrences_of_digit[char_index(digit)] += 1;
            }
        }
    }
    n_occurrences_of_digit
}

fn occurs_n_times(
    n_occurrences_of_digit: &[usize; 7],
    target_n: usize,
) -> impl Iterator<Item = char> + '_ {
    n_occurrences_of_digit
        .iter()
        .enumerate()
        .filter(move |(_, n)| **n == target_n)
        .map(|(index, _)| char::from_u32('a' as u32 + index as u32).unwrap())
}

/*
    0:      1:      2:      3:      4:
 aaaa    ....    aaaa    aaaa    ....
b    c  .    c  .    c  .    c  b    c
b    c  .    c  .    c  .    c  b    c
 ....    ....    dddd    dddd    dddd
e    f  .    f  e    .  .    f  .    f
e    f  .    f  e    .  .    f  .    f
 gggg    ....    gggg    gggg    ....

  5:      6:      7:      8:      9:
 aaaa    aaaa    aaaa    aaaa    aaaa
b    .  b    .  .    c  b    c  b    c
b    .  b    .  .    c  b    c  b    c
 dddd    dddd    ....    dddd    dddd
.    f  e    f  .    f  e    f  .    f
.    f  e    f  .    f  e    f  .    f
 gggg    gggg    ....    gggg    gggg";

digit, length
0 -> 6
1 -> 2 // unique
2 -> 5
3 -> 5
4 -> 4 // unique
5 -> 5
6 -> 6
7 -> 3 // unique
8 -> 7 // unique
9 -> 6
 * */
fn parse_digit(
    input: &[DigitDisplay],
    digit_display: &DigitDisplay,
    occurrences: &mut Option<[usize; 7]>,
) -> usize {
    match digit_display.num_segments() {
        2 => 1,
        3 => 7,
        4 => 4,
        5 => {
            if occurrences.is_none() {
                *occurrences = Some(count_segment_occurrences(input));
            }
            // b is the only one that occurs 6 times
            let b = occurs_n_times(&occurrences.unwrap(), 6).next().unwrap();
            if digit_display.contains(b) {
                // in 2, 3, 5, only 5 contains the segment b
                5
            } else {
                let f = occurs_n_times(&occurrences.unwrap(), 9).next().unwrap();
                if digit_display.contains(f) {
                    // only 3 contains the segment f
                    3
                } else {
                    2
                }
            }
        }
        6 => {
            if occurrences.is_none() {
                *occurrences = Some(count_segment_occurrences(input));
            }
            // e  is the only one that occurs 4 times
            let e = occurs_n_times(&occurrences.unwrap(), 4).next().unwrap();
            if !digit_display.contains(e) {
                // in 0, 6, 9, only 09 does not contain the segment e
                9
            } else {
                // The digits of one are c and f (in an unknown order
                let mut c_and_f = input
                    .iter()
                    .find(|s| s.num_segments() == 2)
                    .unwrap()
                    .chars();
                let c_or_f = c_and_f.next().unwrap();
                let f_or_c = c_and_f.next().unwrap();
                if digit_display.contains(c_or_f) && digit_display.contains(f_or_c) {
                    // 0 contains both digits of one
                    0
                } else {
                    6
                }
            }
        }
        7 => 8,
        _ => panic!("incorrect display string"),
    }
}

fn parse_line(line: &Input) -> usize {
    let mut n = 0;
    let mut occurrences = None;
    for (power_of_10, digit_display) in line.output.iter().rev().enumerate() {
        let parsed = parse_digit(&line.digits, digit_display, &mut occurrences);
        n += parsed as usize * 10_i32.pow(power_of_10 as u32) as usize;
    }
    n
}

#[aoc(day08, part2)]
fn part2(data: &[Input]) -> usize {
    data.iter().map(parse_line).sum()
}

#[cfg(test)]
mod tests {
    fn input() -> Vec<Input> {
        parse_input(include_str!("../input/2021/day8.txt"))
    }
    const E08AMPLE_INPUT_STR: &'static str =
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";
    fn example_input() -> Vec<Input> {
        parse_input(E08AMPLE_INPUT_STR)
    }
    use super::*;
    #[test]
    fn test_part1_given_example_input() {
        assert_eq!(part1(&example_input()), 26)
    }
    #[test]
    fn test_part1() {
        assert_eq!(part1(&input()), 445)
    }
    #[test]
    fn test_part2_given_example_input() {
        assert_eq!(part2(&example_input()), 61229)
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(&input()), 1043101)
    }
}
