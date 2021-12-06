use aoc_runner_derive::{aoc, aoc_generator};

struct Input {
    values: Vec<u16>,
    n_bits: usize,
}

#[aoc_generator(day03)]
fn parse_input(data: &str) -> Input {
    Input {
        values: data
            .lines()
            .map(|s| u16::from_str_radix(s, 2).unwrap())
            .collect::<Vec<_>>(),
        n_bits: data.lines().next().unwrap().chars().count(),
    }
}

fn count_bits(values: &[u16]) -> [u16; 16] {
    values.iter().fold([0; 16], |mut counts, i| {
        for (bit_index, count) in counts.iter_mut().enumerate() {
            *count += i >> bit_index & 1;
        }
        counts
    })
}

fn aggregate_bit_counts(bit_counts: &[u16], pred: impl Fn(u16) -> bool) -> usize {
    bit_counts
        .iter()
        .enumerate()
        .fold(0, |mut value, (index, bit_count)| {
            if pred(*bit_count) {
                value |= 1 << index;
            }
            value
        })
}

fn negate_n_bits(x: usize, n_bits: usize) -> usize {
    let relevant_bits_mask = !(!0 << n_bits);
    (!x) & relevant_bits_mask
}

#[aoc(day03, part1)]
fn part1(data: &Input) -> usize {
    let len = data.values.len();
    let bit_counts = count_bits(&data.values);
    let gamma_rate = aggregate_bit_counts(&bit_counts, |bit_count| bit_count as usize > (len / 2));
    let epsilon_rate = negate_n_bits(gamma_rate, data.n_bits);
    epsilon_rate * gamma_rate
}

fn get_rating(data: &Input, pred: impl Fn(u16, usize) -> bool) -> u16 {
    let mut kept = data.values.clone();
    for top_bit_index in (0..data.n_bits).rev() {
        let bit_count = count_bits(&kept)[top_bit_index];
        let should_keep_bit = pred(bit_count, kept.len());
        kept = kept
            .iter()
            .partition(|value| {
                let bit_set = ((**value >> top_bit_index) & 1) == 1;
                bit_set == should_keep_bit
            })
            .0;
        if kept.len() == 1 {
            return kept[0];
        }
    }
    unreachable!("Prerequisite: we will find exactly one such number")
}

#[aoc(day03, part2)]
fn part2(data: &Input) -> usize {
    let oxygen_generator_rating =
        get_rating(data, |bit_count, len| bit_count as usize * 2 >= len) as usize;
    let co2_generator_rating =
        get_rating(data, |bit_count, len| bit_count as usize * 2 < len) as usize;
    oxygen_generator_rating * co2_generator_rating
}

#[cfg(test)]
mod tests {
    fn input() -> Input {
        parse_input(include_str!("../input/2021/day3.txt"))
    }
    const EXAMPLE_INPUT_STR: &'static str = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";
    fn example_input() -> Input {
        parse_input(EXAMPLE_INPUT_STR)
    }
    use super::*;
    #[test]
    fn test_part1_given_example_input() {
        assert_eq!(part1(&example_input()), 198)
    }
    #[test]
    fn test_part1() {
        assert_eq!(part1(&input()), 3429254)
    }
    #[test]
    fn test_part2_given_example_input() {
        assert_eq!(part2(&example_input()), 230)
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(&input()), 5410338)
    }
}
