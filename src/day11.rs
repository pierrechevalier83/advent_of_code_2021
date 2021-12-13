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

mod grid {
    use super::neighbours;
    pub(super) struct Octopi(Vec<u8>);
    impl Octopi {
        pub(super) fn from_data(data: &[u8]) -> Self {
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
}

mod buckets {
    use super::neighbours;

    pub(super) struct Octopi {
        indices: Vec<bit_set::BitSet>,
    }

    impl Octopi {
        pub(super) fn from_data(data: &[u8]) -> Self {
            let mut indices = std::iter::repeat(bit_set::BitSet::new())
                .take(11)
                .collect::<Vec<_>>();
            for (i, d) in data.iter().enumerate() {
                indices[*d as usize].insert(i);
            }

            Self { indices }
        }
    }

    impl Iterator for Octopi {
        type Item = usize;
        fn next(&mut self) -> Option<Self::Item> {
            for i in (0..10).rev() {
                self.indices.swap(i, i + 1);
            }
            self.indices[0] = bit_set::BitSet::new();
            let mut about_to_flash = self.indices[10].clone();
            let mut already_flashed = bit_set::BitSet::new();
            while !about_to_flash.is_empty() {
                already_flashed = about_to_flash.union(&already_flashed).collect();
                //about_to_flash = about_to_flash
                about_to_flash
                    .iter()
                    // Each octopus can flash at most once
                    //.filter(|&octopus| have_flashed.insert(octopus))
                    .flat_map(neighbours)
                    .for_each(|neighbour_index| {
                        for val in 0..10 {
                            if self.indices[val].remove(neighbour_index) {
                                self.indices[val + 1].insert(neighbour_index);
                                break;
                            }
                        }
                    });
                about_to_flash = self.indices[10].difference(&already_flashed).collect();
            }
            self.indices.swap(0, 10);

            Some(self.indices[0].len())
        }
    }
}

#[aoc(day11, part1, Grid)]
fn part1_grid(data: &[u8]) -> usize {
    grid::Octopi::from_data(data).take(100).sum()
}

#[aoc(day11, part1, Buckets)]
fn part1_buckets(data: &[u8]) -> usize {
    buckets::Octopi::from_data(data).take(100).sum()
}

#[aoc(day11, part2, Grid)]
fn part2_grid(data: &[u8]) -> usize {
    // We want a one-based result
    1 + grid::Octopi::from_data(data)
        .enumerate()
        .find(|(_index, count)| *count == data.len())
        .unwrap()
        .0
}

#[aoc(day11, part2, Buckets)]
fn part2_buckets(data: &[u8]) -> usize {
    // We want a one-based result
    1 + buckets::Octopi::from_data(data)
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
        assert_eq!(part1_grid(&example_input()), 1656);
        assert_eq!(part1_buckets(&example_input()), 1656)
    }
    #[test]
    fn test_part1() {
        assert_eq!(part1_grid(&input()), 1743);
        assert_eq!(part1_buckets(&input()), 1743)
    }
    #[test]
    fn test_part2_given_example_input() {
        assert_eq!(part2_grid(&example_input()), 195);
        assert_eq!(part2_buckets(&example_input()), 195)
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2_grid(&input()), 364);
        assert_eq!(part2_buckets(&input()), 364)
    }
}
