use aoc_runner_derive::{aoc, aoc_generator};
use pathfinding::directed::dijkstra::dijkstra;
use std::str::FromStr;

struct Matrix {
    digits: Vec<u8>,
    n_cols: usize,
}

impl FromStr for Matrix {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let n_cols = s
            .lines()
            .next()
            .ok_or("Expected non empty first line!")?
            .chars()
            .count();
        let digits = s
            .chars()
            .filter(|c| !c.is_ascii_whitespace())
            .map(|c| c.to_digit(10).unwrap() as u8)
            .collect();
        Ok(Self { n_cols, digits })
    }
}

impl Matrix {
    fn tile_one_dimension(vec: &mut Vec<u8>, n: usize, pattern: &[u8]) {
        for i in 0..n {
            vec.extend(pattern.iter().map(|&x| 1 + (x + i as u8 - 1) % 9));
        }
    }
    fn tile(&self, n: usize) -> Self {
        let mut digits = Vec::with_capacity(self.digits.len() * n * n);
        {
            let mut tile_right = Vec::with_capacity(self.digits.len() * n);
            for row in self.digits.chunks(self.n_cols) {
                Self::tile_one_dimension(&mut tile_right, n, row);
            }
            Self::tile_one_dimension(&mut digits, n, &tile_right);
        }
        Self {
            n_cols: n * self.n_cols,
            digits,
        }
    }
    fn shortest_path(&self) -> u32 {
        let start = 0;
        let successors = |&index: &usize| {
            if index % self.n_cols != 0 {
                self.digits
                    .get(index - 1)
                    .map(|cost| (index - 1, *cost as u32))
            } else {
                None
            }
            .into_iter()
            .chain(
                if index % self.n_cols != self.n_cols - 1 {
                    self.digits
                        .get(index + 1)
                        .map(|cost| (index + 1, *cost as u32))
                } else {
                    None
                }
                .into_iter(),
            )
            .chain(
                self.digits
                    .get(index - self.n_cols)
                    .map(|cost| (index - self.n_cols, *cost as u32))
                    .into_iter(),
            )
            .chain(
                self.digits
                    .get(index + self.n_cols)
                    .map(|cost| (index + self.n_cols, *cost as u32))
                    .into_iter(),
            )
        };
        let success = |index: &usize| *index == self.digits.len() - 1;
        dijkstra(&start, successors, success).unwrap().1
    }
}

#[aoc_generator(day15)]
fn parse_input(data: &str) -> Matrix {
    data.parse().unwrap()
}

#[aoc(day15, part1)]
fn part1(data: &Matrix) -> u32 {
    data.shortest_path()
}

#[aoc(day15, part2)]
fn part2(data: &Matrix) -> u32 {
    data.tile(5).shortest_path()
}

#[cfg(test)]
mod tests {
    fn input() -> Matrix {
        parse_input(include_str!("../input/2021/day15.txt"))
    }
    const EXAMPLE_INPUT_STR: &'static str = "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";
    fn example_input() -> Matrix {
        parse_input(EXAMPLE_INPUT_STR)
    }
    use super::*;
    #[test]
    fn test_part1_given_example_input() {
        assert_eq!(part1(&example_input()), 40)
    }
    #[test]
    fn test_part1() {
        assert_eq!(part1(&input()), 523)
    }
    #[test]
    fn test_part2_given_example_input() {
        assert_eq!(part2(&example_input()), 315)
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(&input()), 2876)
    }
}
