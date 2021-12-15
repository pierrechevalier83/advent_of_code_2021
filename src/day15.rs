use aoc_runner_derive::{aoc, aoc_generator};
use petgraph::{algo::astar, graph::Graph, graph::NodeIndex, visit::EdgeRef, Directed};
use std::str::FromStr;

struct Matrix {
    digits: Vec<u32>,
    n_cols: usize,
}

impl FromStr for Matrix {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let n_cols = s
            .lines()
            .next()
            .ok_or("Expected non emptu first line!")?
            .chars()
            .count();
        let digits = s
            .chars()
            .filter(|c| !c.is_ascii_whitespace())
            .map(|c| c.to_digit(10).unwrap())
            .collect();
        Ok(Self { n_cols, digits })
    }
}

impl Matrix {
    fn as_pet_graph(&self) -> Graph<(), (), Directed, usize> {
        let mut edges = self
            .digits
            .chunks(self.n_cols)
            .enumerate()
            .flat_map(move |(i, row)| {
                row.iter().enumerate().flat_map(move |(j, _x)| {
                    let index = i * self.n_cols + j;
                    let left = if j > 1 {
                        Some((index, index - 1))
                    } else {
                        None
                    };
                    let right = if j < self.n_cols - 1 {
                        Some((index, index + 1))
                    } else {
                        None
                    };
                    let up = if index > self.n_cols {
                        Some((index, index - self.n_cols))
                    } else {
                        None
                    };
                    let down = if index >= self.digits.len() - self.n_cols {
                        None
                    } else {
                        Some((index, index + self.n_cols))
                    };
                    left.into_iter()
                        .chain(
                            right
                                .into_iter()
                                .chain(up.into_iter().chain(down.into_iter())),
                        )
                        .collect::<Vec<_>>()
                })
            })
            .collect::<Vec<_>>();
        edges.sort_unstable();
        edges.dedup();
        Graph::from_edges(&edges)
    }
    fn tile_one_dimension<'a>(vec: &mut Vec<u32>, n: usize, pattern: &[u32]) {
        for i in 0..n {
            vec.extend(pattern.iter().map(|&x| 1 + (x + i as u32 - 1) % 9));
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
        let g = self.as_pet_graph();
        let start = NodeIndex::new(0);
        let f = NodeIndex::new(g.node_count() - 1);
        astar(
            &g,
            start,
            |finish| finish == f,
            |e| {
                let res = self.digits[e.target().index()];
                res
            },
            |node| (node.index() / self.n_cols as usize + node.index() % self.n_cols  as usize) as u32,
        )
        .unwrap()
        .0
        // Dynamic programming:
        //
        // Scanning from the back, row by row,
        // fill the shortest path to get to this point.
        // It is the minimum of the total to the right and the total to the bottom.
        // O(n) time,
        // O(n_cols) extra space
        /*
        let mut down_row = self
            .digits
            .iter()
            .rev()
            .take(self.n_cols)
            .copied()
            .collect::<Vec<_>>();
        let mut right_value = None;
        let mut shortest_costs_going_only_right_and_left = self
            .digits
            .iter()
            .rev()
            .enumerate()
            .map(|(index, &digit)| {
                if index % self.n_cols == 0 {
                    // No right neighbour
                    right_value = None;
                }
                let dp = digit
                    + if index >= self.n_cols {
                        down_row[index % self.n_cols].min(right_value.unwrap_or(u32::MAX))
                    } else {
                        // bottom row in input matrix
                        // No down row
                        right_value.unwrap_or(0)
                    };
                down_row[index % self.n_cols] = dp;
                right_value = Some(dp);
                dp
            })
            .collect::<Vec<_>>();
        shortest_costs_going_only_right_and_left.reverse();

        for _ in 0..9 {
            let prev_shortest = shortest_costs_going_only_right_and_left.clone();
            shortest_costs_going_only_right_and_left
                .iter_mut()
                .enumerate()
                .for_each(|(index, x)| {
                    if index % (self.n_cols) != 0 {
                        // look left
                        *x = (*x).min(prev_shortest[index - 1]);
                    }
                    if index >= self.n_cols {
                        // look up
                        *x = (*x).min(prev_shortest[index - self.n_cols]);
                    }
                });
        }
        shortest_costs_going_only_right_and_left[0] - self.digits[0]
        */
    }
}

/* loop up
 * 1.111....
 * 1.1.1....
 * 1.1.1....
 * 111.11...
 * .....1...
 * .....1111
 *
 * first estimate
 * 14  .  6  5  4
 * 13  . 21 12  3
 * 12  . 20 11  2
 * 11 10 11 10 1
 *
 * // do 9 passes where the cost becomes the min(self, neighbour up, left, right, down
 *
 * costs
 * 14  . 6  5  4
 * 13  . 7  .  3
 * 12  . 8  .  2
 * 11 10 9  10 1
 *
 * If path goes right, try going up until 9 away or find cheaper option
 * If path goes down, try going left
 * 1...
 * .....1...
 * .....1121
 * */

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
