use std::str::FromStr;

use aoc_runner_derive::{aoc, aoc_generator};

enum Command {
    Forward,
    Down,
    Up,
}

struct Move {
    command: Command,
    value: usize,
}

impl FromStr for Move {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (first, second) = s
            .split_once(" ")
            .ok_or("Unexpected line: expected two space separated words")?;
        let command = match first {
            "forward" => Command::Forward,
            "down" => Command::Down,
            "up" => Command::Up,
            _ => {
                return Err("Unexpected command");
            }
        };
        let value: usize = second.parse().or(Err("Unexpected value"))?;
        Ok(Self { command, value })
    }
}

#[aoc_generator(day2)]
fn parse_input(data: &str) -> Vec<Move> {
    data.lines()
        .map(Move::from_str)
        .collect::<Result<Vec<_>, _>>()
        .unwrap()
}

mod part1 {
    use super::*;

    #[derive(Default, Debug)]
    pub(super) struct Position {
        pub(super) depth: usize,
        pub(super) horizontal: usize,
    }

    impl Position {
        pub(super) fn apply_move(&mut self, m: &Move) {
            match m.command {
                Command::Forward => self.horizontal += m.value,
                Command::Down => self.depth += m.value,
                Command::Up => self.depth -= m.value,
            }
        }
        pub(super) fn pretty(&self) -> usize {
            self.depth * self.horizontal
        }
    }
}

#[aoc(day2, part1)]
fn part1(data: &[Move]) -> usize {
    data.iter()
        .fold(part1::Position::default(), |mut pos, m| {
            pos.apply_move(m);
            pos
        })
        .pretty()
}

mod part2 {
    use super::*;

    #[derive(Default, Debug)]
    pub(super) struct Position {
        pub(super) aim: usize,
        pub(super) depth: usize,
        pub(super) horizontal: usize,
    }

    impl Position {
        pub(super) fn apply_move(&mut self, m: &Move) {
            match m.command {
                Command::Forward => {
                    self.depth += self.aim * m.value;
                    self.horizontal += m.value;
                }
                Command::Down => self.aim += m.value,
                Command::Up => self.aim -= m.value,
            }
        }
        pub(super) fn pretty(&self) -> usize {
            self.depth * self.horizontal
        }
    }
}

#[aoc(day2, part2)]
fn part2(data: &[Move]) -> usize {
    data.iter()
        .fold(part2::Position::default(), |mut pos, m| {
            pos.apply_move(m);
            pos
        })
        .pretty()
}

#[cfg(test)]
mod tests {
    fn input() -> Vec<Move> {
        parse_input(include_str!("../input/2021/day2.txt"))
    }
    const EXAMPLE_INPUT_STR: &'static str = "forward 5
down 5
forward 8
up 3
down 8
forward 2";
    fn example_input() -> Vec<Move> {
        parse_input(EXAMPLE_INPUT_STR)
    }
    use super::*;
    #[test]
    fn test_part1_given_example_input() {
        assert_eq!(part1(&example_input()), 150)
    }
    #[test]
    fn test_part1() {
        assert_eq!(part1(&input()), 1746616)
    }
    #[test]
    fn test_part2_given_example_input() {
        assert_eq!(part2(&example_input()), 900)
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(&input()), 1741971043)
    }
}
