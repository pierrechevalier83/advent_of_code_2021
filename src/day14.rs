use aoc_runner_derive::{aoc, aoc_generator};
use std::str::FromStr;

const ALPHABET_SIZE: usize = 26;
const N_LETTER_PAIRS: usize = ALPHABET_SIZE * ALPHABET_SIZE;

#[derive(Clone)]
struct Alphabet {
    // Letters contained in our alphabet
    letters: Vec<char>,
    letter_indices: [usize; ALPHABET_SIZE],
}
impl Alphabet {
    fn uppercase_index(c: char) -> usize {
        c as usize - 'A' as usize
    }
    fn index(&self, c: char) -> usize {
        self.letter_indices[Self::uppercase_index(c)]
    }
    fn pair_index(&self, l: char, r: char) -> usize {
        self.index(l) + self.letters.len() * self.index(r)
    }
}

impl FromStr for Alphabet {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut letters = s
            .chars()
            .filter(|&c| char::is_alphabetic(c))
            .collect::<Vec<_>>();
        letters.sort_unstable();
        letters.dedup();
        let mut letter_indices = [0; ALPHABET_SIZE];
        for (i, c) in letters.iter().enumerate() {
            letter_indices[Self::uppercase_index(*c)] = i;
        }

        Ok(Self {
            letters,
            letter_indices,
        })
    }
}

#[derive(Clone)]
struct Polymerization {
    alphabet: Alphabet,
    polymer_letters: Vec<usize>,
    polymer: Vec<usize>,
    insertion_rules: Vec<Option<char>>,
}

impl FromStr for Polymerization {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let alphabet: Alphabet = s.parse()?;
        let mut lines = s.lines();
        let polymer_chars = lines.next().unwrap().chars().collect::<Vec<_>>();
        let mut polymer_letters = std::iter::repeat(0)
            .take(alphabet.letters.len())
            .collect::<Vec<_>>();
        for c in &polymer_chars {
            polymer_letters[alphabet.index(*c)] += 1;
        }
        let mut polymer = std::iter::repeat(0)
            .take(alphabet.letters.len().pow(2))
            .collect::<Vec<_>>();
        for window in polymer_chars.windows(2) {
            polymer[alphabet.pair_index(window[0], window[1])] += 1;
        }
        lines.next();
        let mut insertion_rules = std::iter::repeat(None)
            .take(alphabet.letters.len().pow(2))
            .collect::<Vec<_>>();
        lines.for_each(|line| {
            let (pair, to_insert) = line.split_once(" -> ").unwrap();
            let mut pair = pair.chars();
            insertion_rules[alphabet.pair_index(pair.next().unwrap(), pair.next().unwrap())] =
                Some(to_insert.chars().next().unwrap());
        });

        Ok(Self {
            alphabet,
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
        for &first in &self.alphabet.letters {
            for &second in &self.alphabet.letters {
                let n_pairs = self.polymer[self.alphabet.pair_index(first, second)];
                if n_pairs > 0 {
                    if let Some(middle) =
                        self.insertion_rules[self.alphabet.pair_index(first, second)]
                    {
                        polymer[self.alphabet.pair_index(first, middle)] += n_pairs;
                        polymer[self.alphabet.pair_index(middle, second)] += n_pairs;
                        self.polymer_letters[self.alphabet.index(middle)] += n_pairs;
                    } else {
                        polymer[self.alphabet.pair_index(first, second)] += n_pairs;
                    }
                }
            }
        }
        let len = self.polymer.len();
        let _ = std::mem::replace(&mut self.polymer, polymer[..len].to_vec());
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
