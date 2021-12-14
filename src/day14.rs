use aoc_runner_derive::{aoc, aoc_generator};
use std::str::FromStr;

const ALPHABET_SIZE: usize = 26;
const N_LETTER_PAIRS: usize = ALPHABET_SIZE * ALPHABET_SIZE;

#[derive(Clone)]
struct Polymerization {
    polymer_letters: [usize; ALPHABET_SIZE],
    polymer: [usize; N_LETTER_PAIRS],
    insertion_rules: [Option<char>; N_LETTER_PAIRS],
}

fn uppercase_index(c: char) -> usize {
    c as usize - 'A' as usize
}

fn pair_index(l: char, r: char) -> usize {
    uppercase_index(l) + ALPHABET_SIZE * uppercase_index(r)
}

impl FromStr for Polymerization {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let polymer_chars = lines.next().unwrap().chars().collect::<Vec<_>>();
        let mut polymer_letters = [0; ALPHABET_SIZE];
        for c in &polymer_chars {
            polymer_letters[uppercase_index(*c)] += 1;
        }
        let mut polymer = [0; N_LETTER_PAIRS];
        for window in polymer_chars.windows(2) {
            polymer[pair_index(window[0], window[1])] += 1;
        }
        lines.next();
        let mut insertion_rules = [None; N_LETTER_PAIRS];
        lines.for_each(|line| {
            let (pair, to_insert) = line.split_once(" -> ").unwrap();
            let mut pair = pair.chars();
            insertion_rules[pair_index(pair.next().unwrap(), pair.next().unwrap())] =
                Some(to_insert.chars().next().unwrap());
        });
        Ok(Polymerization {
            polymer_letters,
            polymer,
            insertion_rules,
        })
    }
}

impl Iterator for Polymerization {
    type Item = ();
    fn next(&mut self) -> Option<Self::Item> {
        let mut polymer = [0; N_LETTER_PAIRS];
        for first in 'A'..'Z' {
            for second in 'A'..'Z' {
                let n_pairs = self.polymer[pair_index(first, second)];
                if n_pairs > 0 {
                    if let Some(middle) = self.insertion_rules[pair_index(first, second)] {
                        polymer[pair_index(first, middle)] += n_pairs;
                        polymer[pair_index(middle, second)] += n_pairs;
                        self.polymer_letters[uppercase_index(middle)] += n_pairs;
                    } else {
                        polymer[pair_index(first, second)] += n_pairs;
                    }
                }
            }
        }
        self.polymer = polymer.clone();
        Some(())
    }
}

#[aoc_generator(day14)]
fn parse_input(data: &str) -> Polymerization {
    data.parse().unwrap()
}

fn max_minus_min_after_step_n(data: &mut Polymerization, n: usize) -> usize {
    data.nth(n - 1).unwrap();
    data.polymer_letters.iter().max().unwrap()
        - data
            .polymer_letters
            .iter()
            .filter(|&c| *c != 0)
            .min()
            .unwrap()
}

#[aoc(day14, part1)]
fn part1(data: &Polymerization) -> usize {
    max_minus_min_after_step_n(&mut data.clone(), 10)
}

#[aoc(day14, part2)]
fn part2(data: &Polymerization) -> usize {
    max_minus_min_after_step_n(&mut data.clone(), 40)
}

#[cfg(test)]
mod tests {
    fn input() -> Polymerization {
        parse_input(include_str!("../input/2021/day14.txt"))
    }
    const EXAMPLE_INPUT_STR: &'static str = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";
    fn example_input() -> Polymerization {
        parse_input(EXAMPLE_INPUT_STR)
    }
    use super::*;
    #[test]
    fn test_part1_given_example_input() {
        assert_eq!(part1(&example_input()), 1588)
    }
    #[test]
    fn test_part1() {
        assert_eq!(part1(&input()), 3058)
    }
    #[test]
    fn test_part2_given_example_input() {
        assert_eq!(part2(&example_input()), 2188189693529)
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(&input()), 3447389044530)
    }
}
