use aoc_runner_derive::{aoc, aoc_generator};
use std::str::FromStr;

#[derive(Debug, Clone)]
struct Player {
    id: u8,
    position: usize,
    score: usize,
}

impl Player {
    fn advance_by(&mut self, n: usize) {
        self.position = 1 + (self.position - 1 + n) % 10;
        self.score += self.position;
    }
}

impl FromStr for Player {
    type Err = &'static str;
    // Example input: "Player 1 starting position: 7"
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            id: s
                .replace("Player ", "")
                .chars()
                .next()
                .unwrap()
                .to_digit(10)
                .unwrap() as u8,

            position: s.chars().last().unwrap().to_digit(10).unwrap() as usize,
            score: 0,
        })
    }
}

#[derive(Default)]
struct DeterministicDie {
    last_roll: Option<usize>,
}

impl Iterator for DeterministicDie {
    type Item = usize;
    fn next(&mut self) -> Option<usize> {
        let next = if let Some(last) = self.last_roll {
            if last == 100 {
                1
            } else {
                last + 1
            }
        } else {
            1
        };
        self.last_roll = Some(next);
        self.last_roll
    }
}

#[derive(Debug, Clone)]
struct Game {
    players: Vec<Player>,
    current_player: usize,
}

impl FromStr for Game {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            players: s.lines().map(|line| line.parse().unwrap()).collect(),
            current_player: 0,
        })
    }
}

impl Game {
    // return
    fn next_round(&mut self, die: &mut impl Iterator<Item = usize>) {
        let advance_by = die.take(3).sum();
        self.players[self.current_player].advance_by(advance_by);
        self.current_player = (self.current_player + 1) % self.players.len();
    }
    fn game_over(&self, target_score: usize) -> bool {
        self.players.iter().any(|p| p.score >= target_score)
    }
}

#[aoc_generator(day21)]
fn parse_input(data: &str) -> Game {
    data.parse().unwrap()
}

#[aoc(day21, part1)]
fn part1(g: &Game) -> usize {
    let mut game = g.clone();
    let mut die = DeterministicDie::default();
    let mut n_rolls = 0;
    let target_score = 1000;
    while !game.game_over(target_score) {
        n_rolls += 3;
        game.next_round(&mut die);
    }
    let non_winning_score = game
        .players
        .iter()
        .find(|p| p.score < 1000)
        .map(|p| p.score)
        .unwrap();
    non_winning_score * n_rolls
}

#[aoc(day21, part2)]
fn part2(data: &Game) -> usize {
    42
}

#[cfg(test)]
mod tests {
    fn input() -> Game {
        parse_input(include_str!("../input/2021/day21.txt"))
    }
    const EXAMPLE_INPUT_STR: &'static str = "Player 1 starting position: 4
Player 2 starting position: 8";
    fn example_input() -> Game {
        parse_input(EXAMPLE_INPUT_STR)
    }
    use super::*;
    #[test]
    fn test_part1_given_example_input() {
        assert_eq!(part1(&example_input()), 739785)
    }
    #[test]
    fn test_part1() {
        assert_eq!(part1(&input()), 551901)
    }
    // TODO:
    /*
    #[test]
    fn test_part2_given_example_input() {
        assert_eq!(part2(&example_input()), 444356092776315)
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(&input()), 0)
    }
    */
}
