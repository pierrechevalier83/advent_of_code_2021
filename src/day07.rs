use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day07)]
fn parse_input(data: &str) -> Vec<isize> {
    data.trim()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect::<Vec<_>>()
}

fn total_fuel_cost_part1(data: &[isize], target: isize) -> usize {
    data.iter().map(|x| (*x - target).abs() as usize).sum()
}

// Binary search to find the least cost (input data must be sorted)
fn find_least_cost(data: &[isize], cost_function: impl Fn(&[isize], isize) -> usize) -> usize {
    let mut bottom = data[0];
    let mut top = data[data.len() - 1];

    let mut bottom_cost = cost_function(data, bottom);
    let mut top_cost = cost_function(data, top);
    while top > bottom + 1 {
        let mid = bottom + (top - bottom) / 2;
        let mid_cost = cost_function(data, mid);
        match top_cost.cmp(&bottom_cost) {
            std::cmp::Ordering::Greater => {
                top = mid;
                top_cost = mid_cost;
            }
            std::cmp::Ordering::Less => {
                bottom = mid;
                bottom_cost = mid_cost;
            }
            std::cmp::Ordering::Equal => {
                return mid_cost;
            }
        }
    }
    bottom_cost.min(top_cost)
}

#[aoc(day07, part1)]
fn part1(data: &[isize]) -> usize {
    let mut data = data.iter().copied().collect::<Vec<_>>();
    data.sort_unstable();
    find_least_cost(&data, total_fuel_cost_part1)
}

fn total_fuel_cost_part2(data: &[isize], target: isize) -> usize {
    data.iter()
        .map(|x| (0..(*x - target).abs()).map(|x| x + 1).sum::<isize>() as usize)
        .sum()
}

#[aoc(day07, part2)]
fn part2(data: &[isize]) -> usize {
    let mut data = data.iter().copied().collect::<Vec<_>>();
    data.sort_unstable();
    find_least_cost(&data, total_fuel_cost_part2)
}

#[cfg(test)]
mod tests {
    fn input() -> Vec<isize> {
        parse_input(include_str!("../input/2021/day7.txt"))
    }
    const EXAMPLE_INPUT_STR: &'static str = "16,1,2,0,4,2,7,1,2,14";
    fn example_input() -> Vec<isize> {
        parse_input(EXAMPLE_INPUT_STR)
    }
    use super::*;
    #[test]
    fn test_part1_given_example_input() {
        assert_eq!(part1(&example_input()), 37)
    }
    #[test]
    fn test_part1() {
        assert_eq!(part1(&input()), 355521)
    }
    #[test]
    fn test_part2_given_example_input() {
        assert_eq!(part2(&example_input()), 168)
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(&input()), 100148777)
    }
}
