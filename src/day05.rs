use aoc_runner_derive::{aoc, aoc_generator};

use std::iter::repeat;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Point {
    x: u16,
    y: u16,
}

impl FromStr for Point {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (l, r) = s.split_once(',').ok_or("Unexpected format for Point")?;
        Ok(Self {
            x: l.parse().map_err(|_| "Unexpected format for Point.x")?,
            y: r.parse().map_err(|_| "Unexpected format for Point.y")?,
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
    if *start < end {
        *start += 1;
    } else if *start > end {
        *start -= 1;
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
            if self.0.start.x == self.0.end.x {
                move_start_towards_end(&mut self.0.start.y, self.0.end.y);
                Some(prev)
            // vertical
            } else if self.0.start.y == self.0.end.y {
                move_start_towards_end(&mut self.0.start.x, self.0.end.x);
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
            if self.0.start.x == self.0.end.x {
                move_start_towards_end(&mut self.0.start.y, self.0.end.y);
                Some(prev)
            // vertical
            } else if self.0.start.y == self.0.end.y {
                move_start_towards_end(&mut self.0.start.x, self.0.end.x);
                Some(prev)
                // diagonal
            } else if (self.0.end.x as i16 - self.0.start.x as i16).abs()
                == (self.0.end.y as i16 - self.0.start.y as i16).abs()
            {
                move_start_towards_end(&mut self.0.start.x, self.0.end.x);
                move_start_towards_end(&mut self.0.start.y, self.0.end.y);
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
    data: Vec<Vec<usize>>,
}

impl Grid {
    fn from_segments(segments: &[Segment], mapping_mode: MappingMode) -> Self {
        let max_x = segments.iter().map(|segment| segment.start.x.max(segment.end.x)).max().unwrap() as usize;
        let max_y = segments.iter().map(|segment| segment.start.y.max(segment.end.y)).max().unwrap() as usize;
        let mut data: Vec<Vec<usize>> = repeat(repeat(0).take(max_y + 1).collect())
            .take(max_x + 1)
            .collect();
        for segment in segments {
            let mut last_segment = None;
            match mapping_mode {
                MappingMode::Orthogonal => {
                    for shorter_segment in OrthogonalSegmentIterator(*segment) {
                        last_segment = Some(shorter_segment);
                        data[shorter_segment.start.x as usize][shorter_segment.start.y as usize] +=
                            1;
                    }
                }
                MappingMode::OrthogonalOrDiagonal => {
                    for shorter_segment in OrthogonalOrDiagonalSegmentIterator(*segment) {
                        last_segment = Some(shorter_segment);
                        data[shorter_segment.start.x as usize][shorter_segment.start.y as usize] +=
                            1;
                    }
                }
            };
            if let Some(last_segment) = last_segment {
                data[last_segment.end.x as usize][last_segment.end.y as usize] += 1;
            }
        }
        Self { data }
    }
    fn count_gt_one(&self) -> usize {
        self.data
            .iter()
            .map(|row| {
                row.iter()
                    .map(|value| if *value > 1 { 1 } else { 0 })
                    .sum::<usize>()
            })
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
