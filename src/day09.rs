use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day09)]
fn parse_input(data: &str) -> Vec<Vec<u8>> {
    data.lines()
        .map(|s| s.chars().map(|c| c.to_digit(10).unwrap() as u8).collect())
        .collect::<Vec<_>>()
}

fn neighbours(
    (x, y): (usize, usize),
    data: &[Vec<u8>],
) -> impl Iterator<Item = ((usize, usize), u8)> + '_ {
    [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]
        .into_iter()
        .filter_map(move |coord| {
            data.get(coord.0)
                .and_then(|line| line.get(coord.1))
                .map(move |height| (coord, *height))
        })
}

fn find_low_points(data: &[Vec<u8>]) -> impl Iterator<Item = ((usize, usize), u8)> + '_ {
    data.iter().enumerate().flat_map(move |(i, line)| {
        line.iter()
            .enumerate()
            .filter(move |(j, height)| {
                neighbours((i, *j), data).all(|(_, ref neighbour)| neighbour > height)
            })
            .map(move |(j, height)| ((i, j), *height))
    })
}

#[aoc(day09, part1)]
fn part1(data: &[Vec<u8>]) -> usize {
    find_low_points(data)
        .map(|(_coord, height)| height as usize + 1)
        .sum()
}

fn basin_length(
    point: ((usize, usize), u8),
    data: &[Vec<u8>],
    seen: &mut std::collections::HashSet<(usize, usize)>,
) -> usize {
    let (coord, _) = point;
    let mut boundary = vec![coord];
    while !boundary.is_empty() {
        for coord in &boundary {
            seen.insert(*coord);
        }
        boundary = boundary
            .iter()
            .flat_map(|&coord| {
                neighbours(coord, data)
                    .filter(|(neighbour_coord, _)| !seen.contains(neighbour_coord))
                    .filter(|point| point.1 != 9)
                    .map(|(coord, _)| coord)
            })
            .collect();
    }

    seen.len()
}

#[aoc(day09, part2)]
fn part2(data: &[Vec<u8>]) -> usize {
    let mut heap = find_low_points(data)
        .map(|point| {
            let mut seen = std::collections::HashSet::new();
            basin_length(point, data, &mut seen)
        })
        .collect::<std::collections::BinaryHeap<_>>();
    let top_3 = [
        heap.pop().unwrap_or(0),
        heap.pop().unwrap_or(0),
        heap.pop().unwrap_or(0),
    ];
    top_3.iter().product()
}

#[cfg(test)]
mod tests {
    fn input() -> Vec<Vec<u8>> {
        parse_input(include_str!("../input/2021/day9.txt"))
    }
    const EXAMPLE_INPUT_STR: &'static str = "2199943210
3987894921
9856789892
8767896789
9899965678";
    fn example_input() -> Vec<Vec<u8>> {
        parse_input(EXAMPLE_INPUT_STR)
    }
    use super::*;
    #[test]
    fn test_part1_given_example_input() {
        assert_eq!(part1(&example_input()), 15)
    }
    #[test]
    fn test_part1() {
        assert_eq!(part1(&input()), 456)
    }
    #[test]
    fn test_part2_given_example_input() {
        assert_eq!(part2(&example_input()), 1134)
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(&input()), 1047744)
    }
}
