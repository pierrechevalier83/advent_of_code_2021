use aoc_runner_derive::{aoc, aoc_generator};

use std::iter::repeat;
use std::str::FromStr;

const BINGO_GRID_COLS: usize = 5;
const BINGO_GRID_ROWS: usize = 5;

#[derive(Debug, Clone)]
struct Board {
    data: [[u8; BINGO_GRID_COLS]; BINGO_GRID_ROWS],
}

impl FromStr for Board {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut data = [[0; BINGO_GRID_ROWS]; BINGO_GRID_COLS];
        s.lines()
            .filter(|line| !line.is_empty())
            .enumerate()
            .map(|(row_index, line)| {
                line.split_whitespace()
                    .enumerate()
                    .map(|(col_index, word)| {
                        data[row_index][col_index] = word
                            .parse::<u8>()
                            .map_err(|_| "Failed to parse board value")?;
                        Ok(())
                    })
                    .collect::<Result<_, _>>()
            })
            .collect::<Result<_, _>>()?;
        Ok(Self { data })
    }
}

#[derive(Debug)]
struct VisitedBoard {
    row_first: u32,
    col_first: u32,
}

impl VisitedBoard {
    fn new() -> Self {
        Self {
            row_first: 0,
            col_first: 0,
        }
    }
    fn row_first_index(row: usize, col: usize) -> usize {
        row * BINGO_GRID_COLS + col
    }
    fn col_first_index(row: usize, col: usize) -> usize {
        col * BINGO_GRID_ROWS + row
    }
    fn visit(&mut self, row: usize, col: usize) {
        self.row_first |= 1 << Self::row_first_index(row, col);
        self.col_first |= 1 << Self::col_first_index(row, col);
    }
    fn winning(&self) -> bool {
        for index in 0..BINGO_GRID_ROWS {
            if (self.row_first >> (5 * index)).trailing_ones() as usize >= BINGO_GRID_COLS {
                return true;
            }
        }
        for index in 0..BINGO_GRID_COLS {
            if (self.col_first >> (5 * index)).trailing_ones() as usize >= BINGO_GRID_ROWS {
                return true;
            }
        }
        false
    }
    fn row_first(&self) -> [[bool; BINGO_GRID_COLS]; BINGO_GRID_ROWS] {
        let mut ret = [[false; BINGO_GRID_COLS]; BINGO_GRID_ROWS];
        for (row_index, row) in ret.iter_mut().enumerate() {
            for (col_index, val) in row.iter_mut().enumerate() {
                *val = (self.row_first >> Self::row_first_index(row_index, col_index)) & 1 == 1;
            }
        }
        ret
    }
}

#[derive(Debug, Clone, Copy)]
struct Position {
    board_index: usize,
    row_index: usize,
    col_index: usize,
}

const MAX_BINGO_VALUE: usize = 100;

#[derive(Debug, Clone)]
struct BingoInput {
    nums: Vec<u8>,
    boards: Vec<Board>,
    positions: Vec<Vec<Position>>,
}

impl FromStr for BingoInput {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (nums_str, boards_str) = s.split_once("\n\n").ok_or("Surprising input shape")?;
        let boards = boards_str
            .split("\n\n")
            .map(Board::from_str)
            .collect::<Result<Vec<_>, _>>()?;
        let mut positions: Vec<Vec<Position>> = repeat(Vec::with_capacity(100))
            .take(MAX_BINGO_VALUE)
            .collect();
        boards.iter().enumerate().for_each(|(board_index, board)| {
            board.data.iter().enumerate().for_each(|(row_index, row)| {
                row.iter().enumerate().for_each(|(col_index, val)| {
                    positions[*val as usize].push(Position {
                        board_index,
                        row_index,
                        col_index,
                    });
                })
            })
        });

        Ok(Self {
            nums: nums_str
                .split(",")
                .map(|s| s.parse().map_err(|_| "Incorrect value in nums"))
                .collect::<Result<_, _>>()?,
            boards,
            positions,
        })
    }
}

#[derive(Debug)]
struct Bingo {
    input: BingoInput,
    visited: Vec<VisitedBoard>,
    index_to_draw_next: usize,
    won: Vec<bool>,
}

impl Bingo {
    fn from_input(input: &BingoInput) -> Self {
        Self {
            input: input.clone(),
            visited: input.boards.iter().map(|_| VisitedBoard::new()).collect(),
            index_to_draw_next: 0,
            won: repeat(false).take(input.boards.len()).collect(),
        }
    }
    fn winning_boards(&self) -> Vec<usize> {
        self.visited
            .iter()
            .enumerate()
            .filter(|(board_index, _)| {
                // If we already won, no need to count it again
                !self.won[*board_index]
            })
            .filter_map(|(board_index, board)| {
                let winning = board.winning();
                if winning {
                    Some(board_index)
                } else {
                    None
                }
            })
            .collect()
    }
    fn score(&self, winning_boards: Vec<usize>) -> usize {
        winning_boards
            .iter()
            .map(|winning_board| {
                self.visited[*winning_board]
                    .row_first()
                    .iter()
                    .enumerate()
                    .map(|(row_index, row)| {
                        row.iter()
                            .enumerate()
                            .map(|(col_index, col)| {
                                if *col {
                                    0
                                } else {
                                    self.input.boards[*winning_board].data[row_index][col_index]
                                        as usize
                                }
                            })
                            .sum::<usize>()
                    })
                    .sum::<usize>()
            })
            .sum::<usize>()
    }
}

impl Iterator for Bingo {
    type Item = (usize, Vec<usize>);
    fn next(&mut self) -> Option<Self::Item> {
        if self.index_to_draw_next >= self.input.nums.len() {
            return None;
        }
        let draw = self.input.nums[self.index_to_draw_next];
        for pos in &self.input.positions[draw as usize] {
            self.visited[pos.board_index].visit(pos.row_index, pos.col_index)
        }
        self.index_to_draw_next += 1;
        let winning_boards = self.winning_boards();
        for winning_board in &winning_boards {
            self.won[*winning_board] = true;
        }
        Some((draw as usize, winning_boards))
    }
}

#[aoc_generator(day04)]
fn parse_input(data: &str) -> BingoInput {
    BingoInput::from_str(data).unwrap()
}

#[aoc(day04, part1)]
fn part1(data: &BingoInput) -> usize {
    let mut bingo = Bingo::from_input(data);
    let (draw, winning) = bingo
        .find(|(_, winning_boards)| !winning_boards.is_empty())
        .unwrap();
    bingo.score(winning) * draw
}

#[aoc(day04, part2)]
fn part2(data: &BingoInput) -> usize {
    let mut bingo = Bingo::from_input(data);
    let (mut draw, mut winning) = bingo
        .find(|(_, winning_boards)| !winning_boards.is_empty())
        .unwrap();
    while !bingo.won.iter().all(|x| *x) {
        (draw, winning) = bingo
            .find(|(_, winning_boards)| !winning_boards.is_empty())
            .unwrap();
    }
    bingo.score(winning) * draw
}

#[cfg(test)]
mod tests {
    fn input() -> BingoInput {
        parse_input(include_str!("../input/2021/day4.txt"))
    }
    const E04AMPLE_INPUT_STR: &'static str =
        "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";
    fn example_input() -> BingoInput {
        parse_input(E04AMPLE_INPUT_STR)
    }
    use super::*;
    #[test]
    fn test_part1_given_example_input() {
        assert_eq!(part1(&example_input()), 4512)
    }
    #[test]
    fn test_part1() {
        assert_eq!(part1(&input()), 16716)
    }
    #[test]
    fn test_part2_given_example_input() {
        assert_eq!(part2(&example_input()), 1924)
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(&input()), 4880)
    }
}
