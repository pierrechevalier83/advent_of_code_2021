use aoc_runner_derive::{aoc, aoc_generator};
use drawille;
use std::str::FromStr;

const NINE_BITS_MASK: usize = 0b111111111;

#[derive(Debug, Clone)]
struct Input {
    enhancement_algorithm: [bool; 512],
    image: Vec<bool>,
    n_cols: usize,
    parity: bool,
}

impl FromStr for Input {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let mut enhancement_algorithm = [false; 512];
        for (i, c) in lines.next().unwrap().chars().enumerate() {
            assert!(c == '.' || c == '#');
            enhancement_algorithm[i] = c == '#';
        }
        // empty line
        lines.next();
        let mut image = Vec::new();
        let mut n_cols = 0;
        for (i, row) in lines.enumerate() {
            if i == 0 {
                n_cols = row.trim().chars().count();
            }
            for c in row.chars() {
                assert!(c == '.' || c == '#');
                image.push(c == '#');
            }
        }
        Ok(Input {
            enhancement_algorithm,
            image,
            n_cols,
            parity: true,
        })
    }
}

fn row_first_index(n_cols: isize, r: isize, c: isize) -> isize {
    r * n_cols + c
}

impl Input {
    fn n_rows(&self) -> usize {
        self.image.len() / self.n_cols
    }
    fn enhance(&self) -> Self {
        let mut enhanced = Self {
            enhancement_algorithm: self.enhancement_algorithm,
            image: Vec::with_capacity((self.n_cols + 2) * (self.n_rows() + 2)),
            n_cols: self.n_cols + 2,
            parity: !self.parity
        };

        for row in -1..self.n_rows() as isize + 1 {
            for col in -1..self.n_cols as isize + 1 {
                let mut bits: usize;
                if self.enhancement_algorithm[0] {
                    // All unexplored territory switches bewteen all 0s when even and all 1s when odd
                    if self.parity {
                        bits = 0;
                    } else {
                        bits = NINE_BITS_MASK;
                    }
                } else {
                    bits = 0;
                }
                for r in -1..=1 {
                    for c in -1..=1 {
                        if row + r >= 0
                            && col + c >= 0
                            && col + c < self.n_cols as isize
                            && row + r < self.n_rows() as isize
                        {
                            let index = row_first_index(self.n_cols as isize, row + r, col + c);
                            let bit_index = 8 - row_first_index(3, r + 1, c + 1);
                            if self.image[index as usize] {
                                bits |= 1 << bit_index;
                            } else {
                                bits &= !(1 << bit_index);
                                bits &= NINE_BITS_MASK;
                            }
                        }
                    }
                }
                enhanced.image.push(self.enhancement_algorithm[bits]);
            }
        }
        enhanced
    }
    fn num_lit_pixels(&self) -> usize {
        self.image.iter().filter(|&px| *px).count()
    }
    #[allow(unused)]
    fn draw(self) -> Self {
        let mut canvas = drawille::Canvas::new(self.n_cols as u32, self.n_rows() as u32);
        (0..self.n_rows()).for_each(|x| {
            (0..self.n_cols).for_each(|y| {
                if self.image
                    [row_first_index(self.n_cols as isize, x as isize, y as isize) as usize]
                {
                    canvas.set(y as u32, x as u32);
                }
            })
        });
        println!(
            "{} * {} == {}",
            self.n_cols,
            self.n_rows(),
            self.image.len()
        );
        println!("{}", canvas.frame());
        self
    }
}

#[aoc_generator(day20)]
fn parse_input(data: &str) -> Input {
    data.parse().unwrap()
}

#[aoc(day20, part1)]
fn part1(data: &Input) -> usize {
    data.enhance()
        .enhance()
        .num_lit_pixels()
}

#[aoc(day20, part2)]
fn part2(data: &Input) -> usize {
    let mut d = data.clone();
    for _ in 0..50 {
        d = d.enhance()
    }
    d.num_lit_pixels()
}

#[cfg(test)]
mod tests {
    fn input() -> Input {
        parse_input(include_str!("../input/2021/day20.txt"))
    }
    const EXAMPLE_INPUT_STR: &'static str = "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###";
    fn example_input() -> Input {
        parse_input(EXAMPLE_INPUT_STR)
    }
    use super::*;
    #[test]
    fn test_part1_given_example_input() {
        assert_eq!(part1(&example_input()), 35)
    }
    #[test]
    fn test_part1() {
        assert_eq!(part1(&input()), 5291)
    }
    #[test]
    fn test_part2_given_example_input() {
        assert_eq!(part2(&example_input()), 3351)
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(&input()), 16665)
    }
}
