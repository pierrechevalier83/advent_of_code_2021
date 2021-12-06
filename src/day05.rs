use aoc_runner_derive::{aoc, aoc_generator};

use std::iter::repeat;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Point {
    row: u16,
    col: u16,
}

impl FromStr for Point {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (l, r) = s.split_once(',').ok_or("Unexpected format for Point")?;
        Ok(Self {
            row: l.parse().map_err(|_| "Unexpected format for Point.x")?,
            col: r.parse().map_err(|_| "Unexpected format for Point.y")?,
        })
    }
}

#[derive(Debug, Clone, Copy)]
struct Segment {
    start: Point,
    end: Point,
}

impl FromStr for Segment {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (l, r) = s
            .split_once(" -> ")
            .ok_or("Unexpected format for Segment")?;
        Ok(Self {
            start: l
                .parse()
                .map_err(|_| "Unexpected format for Segment.start")?,
            end: r.parse().map_err(|_| "Unexpected format for Segment.end")?,
        })
    }
}

fn move_start_towards_end(start: &mut u16, end: u16) {
    match (*start).cmp(&end) {
        std::cmp::Ordering::Less => {
            *start += 1;
        }
        std::cmp::Ordering::Greater => {
            *start -= 1;
        }
        _ => {
            // start already at end. Do nothing
        }
    }
}

struct OrthogonalSegmentIterator(Segment);
impl Iterator for OrthogonalSegmentIterator {
    type Item = Segment;
    fn next(&mut self) -> Option<Self::Item> {
        if self.0.start == self.0.end {
            None
        } else {
            let prev = self.0;
            // horizontal
            if self.0.start.row == self.0.end.row {
                move_start_towards_end(&mut self.0.start.col, self.0.end.col);
                Some(prev)
            // vertical
            } else if self.0.start.col == self.0.end.col {
                move_start_towards_end(&mut self.0.start.row, self.0.end.row);
                Some(prev)
            // ignore diagonals
            } else {
                None
            }
        }
    }
}

struct OrthogonalOrDiagonalSegmentIterator(Segment);
impl Iterator for OrthogonalOrDiagonalSegmentIterator {
    type Item = Segment;
    fn next(&mut self) -> Option<Self::Item> {
        if self.0.start == self.0.end {
            None
        } else {
            let prev = self.0;
            // horizontal
            if self.0.start.row == self.0.end.row {
                move_start_towards_end(&mut self.0.start.col, self.0.end.col);
                Some(prev)
            // vertical
            } else if self.0.start.col == self.0.end.col {
                move_start_towards_end(&mut self.0.start.row, self.0.end.row);
                Some(prev)
                // diagonal
            } else if (self.0.end.row as i16 - self.0.start.row as i16).abs()
                == (self.0.end.col as i16 - self.0.start.col as i16).abs()
            {
                move_start_towards_end(&mut self.0.start.row, self.0.end.row);
                move_start_towards_end(&mut self.0.start.col, self.0.end.col);
                Some(prev)
            } else {
                None
            }
        }
    }
}

enum MappingMode {
    Orthogonal,
    OrthogonalOrDiagonal,
}

#[derive(Debug)]
struct Grid {
    // Optimization: allocate some space, but get very fast access in return
    // Benchmarked against HashMap and BTreeMap
    // This is 4-5 times faster than BTreeMap and 2-3 times faster than HashMap
    data: Vec<usize>,
}

fn row_major_index(p: Point, n_cols: usize) -> usize {
    p.row as usize * n_cols + p.col as usize
}

impl Grid {
    fn from_segments(segments: &[Segment], mapping_mode: MappingMode) -> Self {
        let n_cols = segments
            .iter()
            .map(|segment| segment.start.col.max(segment.end.col))
            .max()
            .unwrap() as usize
            + 1;
        let n_rows = segments
            .iter()
            .map(|segment| segment.start.row.max(segment.end.row))
            .max()
            .unwrap() as usize
            + 1;
        let mut data: Vec<usize> = repeat(0).take(n_cols * n_rows).collect();
        for segment in segments {
            let mut last_segment = None;
            match mapping_mode {
                MappingMode::Orthogonal => {
                    for shorter_segment in OrthogonalSegmentIterator(*segment) {
                        last_segment = Some(shorter_segment);
                        data[row_major_index(shorter_segment.start, n_cols)] += 1;
                    }
                }
                MappingMode::OrthogonalOrDiagonal => {
                    for shorter_segment in OrthogonalOrDiagonalSegmentIterator(*segment) {
                        last_segment = Some(shorter_segment);
                        data[row_major_index(shorter_segment.start, n_cols)] += 1;
                    }
                }
            };
            if let Some(last_segment) = last_segment {
                data[row_major_index(last_segment.end, n_cols)] += 1;
            }
        }
        Self { data }
    }
    fn count_gt_one(&self) -> usize {
        self.data
            .iter()
            .map(|value| if *value > 1 { 1 } else { 0 })
            .sum()
    }
}

#[aoc_generator(day05)]
fn parse_input(data: &str) -> Vec<Segment> {
    data.lines()
        .map(Segment::from_str)
        .collect::<Result<Vec<_>, _>>()
        .unwrap()
}

#[aoc(day05, part1)]
fn part1(data: &[Segment]) -> usize {
    Grid::from_segments(data, MappingMode::Orthogonal).count_gt_one()
}

#[aoc(day05, part2)]
fn part2(data: &[Segment]) -> usize {
    let g = Grid::from_segments(data, MappingMode::OrthogonalOrDiagonal);
    g.count_gt_one()
}

#[cfg(test)]
mod tests {
    fn input() -> Vec<Segment> {
        parse_input(include_str!("../input/2021/day5.txt"))
    }
    const EXAMPLE_INPUT_STR: &'static str = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";
    fn example_input() -> Vec<Segment> {
        parse_input(EXAMPLE_INPUT_STR)
    }
    use super::*;
    #[test]
    fn test_part1_given_example_input() {
        assert_eq!(part1(&example_input()), 5)
    }
    #[test]
    fn test_part1() {
        assert_eq!(part1(&input()), 8350)
    }
    #[test]
    fn test_part2_given_example_input() {
        assert_eq!(part2(&example_input()), 12)
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(&input()), 19374)
    }
}
