use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
fn parse_input(data: &str) -> Vec<u16> {
    data.split_terminator('\n')
        .map(|s| s.parse().unwrap())
        .collect::<Vec<_>>()
}

fn count_increases(data: &[u16]) -> usize {
    data.windows(2)
        .filter(|window| window[1] > window[0])
        .count()
}

#[aoc(day1, part1)]
fn part1(data: &[u16]) -> usize {
    count_increases(data)
}

#[aoc(day1, part2)]
fn part2(data: &[u16]) -> usize {
    count_increases(
        &data
            .windows(3)
            .map(|window| window.iter().sum())
            .collect::<Vec<u16>>(),
    )
}

#[cfg(test)]
mod tests {
    fn input() -> Vec<u16> {
        parse_input(include_str!("../input/2021/day1.txt"))
    }
    fn example_input() -> Vec<u16> {
        vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263]
    }
    use super::*;
    #[test]
    fn test_part1_given_example_input() {
        assert_eq!(part1(&example_input()), 7)
    }
    #[test]
    fn test_part1() {
        assert_eq!(part1(&input()), 1184)
    }
    #[test]
    fn test_part2_given_example_input() {
        assert_eq!(part2(&example_input()), 5)
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(&input()), 1158)
    }
}
