use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day11)]
fn parse_input(data: &str) -> Vec<u8> {
    data.lines()
        .flat_map(|s| s.chars().map(|c| c.to_digit(10).unwrap() as u8))
        .collect::<Vec<_>>()
}

const N_COLS: usize = 10;
const N_COLS_ISIZE: isize = N_COLS as isize;

fn row_first_index(row: usize, col: usize) -> usize {
    row * N_COLS + col
}

fn from_row_first_index(index: usize) -> (usize, usize) {
    (index / N_COLS, index % N_COLS)
}

fn neighbours(index: usize) -> impl Iterator<Item = usize> {
    let (point_x, point_y) = from_row_first_index(index);
    [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (1, -1),
        (1, 0),
        (1, 1),
        (0, -1),
        (0, 1),
    ]
    .iter()
    .filter_map(move |(x_offset, y_offset)| {
        match (point_x as isize + x_offset, point_y as isize + y_offset) {
            // At wall
            (-1, _) | (_, -1) | (N_COLS_ISIZE, _) | (_, N_COLS_ISIZE) => None,
            (x, y) => Some(row_first_index(x as usize, y as usize)),
        }
    })
}

struct Octopi(Vec<u8>);

impl Octopi {
    fn from_data(data: &[u8]) -> Self {
        Self(data.to_vec())
    }
}

impl Iterator for Octopi {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        for octopus in self.0.iter_mut() {
            *octopus += 1;
        }
        let mut about_to_flash = self
            .0
            .iter()
            .enumerate()
            .filter_map(|(index, value)| if *value >= 10 { Some(index) } else { None })
            .collect::<Vec<_>>();
        let mut have_flashed = bit_set::BitSet::new();
        while !about_to_flash.is_empty() {
            about_to_flash = about_to_flash
                .iter()
                // Each octopus can flash at most once
                .filter(|&octopus| have_flashed.insert(*octopus))
                .flat_map(|octopus| neighbours(*octopus))
                .filter(|neighbour_index| {
                    if self.0[*neighbour_index] < 10 {
                        self.0[*neighbour_index] += 1;
                    }
                    self.0[*neighbour_index] >= 10
                })
                .collect::<Vec<_>>();
        }

        for index in &have_flashed {
            if self.0[index] >= 10 {
                self.0[index] = 0;
            }
        }

        Some(have_flashed.len())
    }
}

#[aoc(day11, part1)]
fn part1(data: &[u8]) -> usize {
    Octopi::from_data(data).take(100).sum()
}

#[aoc(day11, part2)]
fn part2(data: &[u8]) -> usize {
    // We want a one-based result
    1 + Octopi::from_data(data)
        .enumerate()
        .find(|(_index, count)| *count == data.len())
        .unwrap()
        .0
}

#[cfg(test)]
mod tests {
    fn input() -> Vec<u8> {
        parse_input(include_str!("../input/2021/day11.txt"))
    }
    const EXAMPLE_INPUT_STR: &'static str = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";
    fn example_input() -> Vec<u8> {
        parse_input(EXAMPLE_INPUT_STR)
    }
    use super::*;
    #[test]
    fn test_part1_given_example_input() {
        assert_eq!(part1(&example_input()), 1656)
    }
    #[test]
    fn test_part1() {
        assert_eq!(part1(&input()), 1743)
    }
    #[test]
    fn test_part2_given_example_input() {
        assert_eq!(part2(&example_input()), 195)
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(&input()), 364)
    }
}
