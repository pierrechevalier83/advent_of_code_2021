use aoc_runner_derive::{aoc, aoc_generator};
use std::str::FromStr;

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
struct Point {
    x: usize,
    y: usize,
}

impl FromStr for Point {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.trim().split_once(',').unwrap();
        Ok(Self {
            x: x.parse().unwrap(),
            y: y.parse().unwrap(),
        })
    }
}

#[derive(Debug, Clone, Copy)]
enum Axis {
    X,
    Y,
}

impl FromStr for Axis {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "x" => Ok(Self::X),
            "y" => Ok(Self::Y),
            _ => Err("Can't parse Axis"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Fold {
    axis: Axis,
    position: usize,
}

impl FromStr for Fold {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens = s.trim_start_matches("fold along ").rsplit('=');
        let position = tokens.next().unwrap();
        let axis = tokens.next().unwrap();
        Ok(Self {
            axis: axis.parse().unwrap(),
            position: position.parse().unwrap(),
        })
    }
}

#[derive(Debug, Clone)]
struct InstructionsSheet {
    dots: Vec<Point>,
    folds: Vec<Fold>,
}

impl FromStr for InstructionsSheet {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (points, folds) = s.split_once("\n\n").unwrap();
        Ok(InstructionsSheet {
            dots: points.lines().map(|line| line.parse().unwrap()).collect(),
            folds: folds.lines().map(|line| line.parse().unwrap()).collect(),
        })
    }
}

impl std::fmt::Display for InstructionsSheet {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let n_cols = self.dots.iter().map(|dot| dot.x).max().unwrap() + 1;
        let n_rows = self.dots.iter().map(|dot| dot.y).max().unwrap() + 1;

        write!(
            f,
            "{}",
            (0..n_rows)
                .flat_map(|y| {
                    (0..n_cols)
                        .map(move |x| {
                            if self.dots.contains(&Point { x, y }) {
                                '█'
                            } else {
                                ' '
                            }
                        })
                        .chain(std::iter::once('\n'))
                })
                .collect::<String>()
        )
    }
}

struct FoldIterator {
    instructions: InstructionsSheet,
    fold_index: usize,
}

impl FoldIterator {
    fn new(instructions: &InstructionsSheet) -> Self {
        Self {
            instructions: instructions.clone(),
            fold_index: 0,
        }
    }
}

fn fold_point(pos: usize, dot_coordinate: &mut usize) {
    if *dot_coordinate > pos {
        assert!(*dot_coordinate <= pos * 2);
        *dot_coordinate = 2 * pos - *dot_coordinate
    }
}

impl Iterator for FoldIterator {
    type Item = InstructionsSheet;
    fn next(&mut self) -> Option<Self::Item> {
        if self.fold_index < self.instructions.folds.len() {
            let next_fold = self.instructions.folds[self.fold_index];
            for dot in self.instructions.dots.iter_mut() {
                // fold point
                match (next_fold.axis, next_fold.position) {
                    (Axis::X, pos) => fold_point(pos, &mut dot.x),
                    (Axis::Y, pos) => fold_point(pos, &mut dot.y),
                }
            }
            self.instructions.dots.sort_unstable();
            self.instructions.dots.dedup();
            self.fold_index += 1;
            Some(self.instructions.clone())
        } else {
            None
        }
    }
}

#[aoc_generator(day13)]
fn parse_input(data: &str) -> InstructionsSheet {
    data.parse().unwrap()
}

#[aoc(day13, part1)]
fn part1(data: &InstructionsSheet) -> usize {
    let instructions_sheet = FoldIterator::new(data).next().unwrap();
    instructions_sheet.dots.len()
}

#[aoc(day13, part2)]
fn part2(data: &InstructionsSheet) -> String {
    let instructions = FoldIterator::new(data).last().unwrap();
    format!("\n{}", instructions)
}

#[cfg(test)]
mod tests {
    fn input() -> InstructionsSheet {
        parse_input(include_str!("../input/2021/day13.txt"))
    }
    const EXAMPLE_INPUT_STR: &'static str = "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";
    fn example_input() -> InstructionsSheet {
        parse_input(EXAMPLE_INPUT_STR)
    }
    use super::*;
    #[test]
    fn test_part1_given_example_input() {
        assert_eq!(part1(&example_input()), 17)
    }
    #[test]
    fn test_part1() {
        assert_eq!(part1(&input()), 810)
    }
    #[test]
    fn test_part2_given_example_input() {
        let output = "
█████
█   █
█   █
█   █
█████
";
        assert_eq!(&part2(&example_input()), output)
    }
    #[test]
    fn test_part2() {
        let output = "
█  █ █    ███  █  █ ███   ██  ████ ███ 
█  █ █    █  █ █  █ █  █ █  █ █    █  █
████ █    ███  █  █ ███  █    ███  █  █
█  █ █    █  █ █  █ █  █ █ ██ █    ███ 
█  █ █    █  █ █  █ █  █ █  █ █    █ █ 
█  █ ████ ███   ██  ███   ███ █    █  █
";
        assert_eq!(&part2(&input()), output)
    }
}
