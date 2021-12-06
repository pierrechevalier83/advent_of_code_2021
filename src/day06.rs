use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Clone)]
struct Counts([usize; 9]);

impl Counts {
    fn from_ages(ages: &[u8]) -> Self {
        let mut counts = [0; 9];
        for i in 0..9 {
            counts[i] = ages.iter().filter(|x| **x as usize == i).count();
        }
        Counts(counts)
    }
}

#[aoc_generator(day06)]
fn parse_input(data: &str) -> Counts {
    Counts::from_ages(
        &data
            .trim()
            .split(",")
            .map(|s| s.parse().unwrap())
            .collect::<Vec<_>>(),
    )
}

fn simulate_one_day(counts: &mut Counts) {
    let count_newborns = counts.0[0];
    for i in 0..8 {
        counts.0[i] = counts.0[i + 1];
    }
    counts.0[6] += count_newborns;
    counts.0[8] = count_newborns;
}

fn simulate_n_days(mut counts: Counts, n: usize) -> usize {
    for _ in 0..n {
        simulate_one_day(&mut counts);
    }
    counts.0.iter().sum()
}

#[aoc(day06, part1)]
fn part1(counts: &Counts) -> usize {
    simulate_n_days(counts.clone(), 80)
}

#[aoc(day06, part2)]
fn part2(counts: &Counts) -> usize {
    simulate_n_days(counts.clone(), 256)
}

#[cfg(test)]
mod tests {
    fn input() -> Counts {
        parse_input(include_str!("../input/2021/day6.txt"))
    }
    const EXAMPLE_INPUT_STR: &'static str = "3,4,3,1,2";
    fn example_input() -> Counts {
        parse_input(EXAMPLE_INPUT_STR)
    }
    use super::*;
    #[test]
    fn test_part1_given_example_input() {
        assert_eq!(part1(&example_input()), 5934)
    }
    #[test]
    fn test_part1() {
        assert_eq!(part1(&input()), 363101)
    }
    #[test]
    fn test_part2_given_example_input() {
        assert_eq!(part2(&example_input()), 26984457539)
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(&input()), 1644286074024)
    }
}
