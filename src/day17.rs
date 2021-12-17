use aoc_runner_derive::{aoc, aoc_generator};
use std::cmp::Ordering;
use std::ops::RangeInclusive;
use std::str::FromStr;

#[derive(Debug, Clone, Eq, PartialEq)]
struct TargetArea {
    x: RangeInclusive<i32>,
    y: RangeInclusive<i32>,
}

impl FromStr for TargetArea {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s
            .trim()
            .trim_start_matches("target area: ")
            .split_once(", ")
            .ok_or("Failed to parse line")?;
        let (x_start, x_end) = x
            .trim_start_matches("x=")
            .split_once("..")
            .ok_or("Failed to parse x")?;
        let (y_start, y_end) = y
            .trim_start_matches("y=")
            .split_once("..")
            .ok_or("Failed to parse y")?;
        Ok(Self {
            x: x_start.parse().unwrap()..=x_end.parse().unwrap(),
            y: y_start.parse().unwrap()..=y_end.parse().unwrap(),
        })
    }
}

impl TargetArea {
    fn contains(&self, (x, y): (i32, i32)) -> bool {
        self.x.contains(&x) && self.y.contains(&y)
    }
    fn was_missed(&self, (x, y): (i32, i32)) -> bool {
        &x > self.x.end() || &y < self.y.start()
    }
}

#[aoc_generator(day17)]
fn parse_input(data: &str) -> TargetArea {
    data.parse().unwrap()
}

// let a(x) be the arithmetic progression from one to x (1 + 2 + ... + x - 1 + x)
// i | x                 | y                 | vx(i)  | vy(i)  |
// 0 | 0                 |  0                | vx     | vy     |
// while vy < apex && vx >= 0
// i | a(vx) - a(vx - i) | a(vy) - a(vy - 1) | vx - i | vy - i |

// Firing the probe with this velocity, can we reach the target?
fn reaches_destination((mut vx, mut vy): (i32, i32), target: &TargetArea) -> bool {
    let (mut x, mut y) = (0, 0);
    // if vy is positive, we can predict the position and velocity once back at sea
    // level in constant time
    if vy > 0 {
        let step = 2 * vy + 1;
        x = arithmetic_progresion_from_one_to(vx) - arithmetic_progresion_from_one_to(vx - step);
        y = 0;
        vx = (vx - step).max(0);
        vy = vy - step;
    }
    // Now y is always negative and decreasing
    // x is always positive and decreasing
    //
    // Note: this could now be done faster with a binary search since we always start with x > 0 and
    // increasing and y negative and decreasing.
    // We can calculate for any step:
    // ```
    // vx(step) = vx0 + step
    // vy(step) = vy0 - step
    // y(step) = y0 - arithmetic_progresion_from_one_to(step) + arithmetic_progresion_from_one_to(-vy0);
    // x(step) = x0 + arithmetic_progresion_from_one_to(step) - arithmetic_progresion_from_one_to(vx0)
    // ```
    while !target.contains((x, y))
        && !target.was_missed((x, y))
        && (vx != 0 || target.x.contains(&x))
    {
        x += vx;
        y += vy;
        vx += match vx.cmp(&0) {
            Ordering::Less => 1,
            Ordering::Greater => -1,
            Ordering::Equal => 0,
        };
        vy -= 1;
        if target.contains((x, y)) {
            return true;
        }
    }
    false
}

fn arithmetic_progresion_from_one_to(x: i32) -> i32 {
    if x < 0 {
        0
    } else {
        (x * (x + 1)) / 2
    }
}

fn every_initial_velocity(target: &TargetArea) -> impl Iterator<Item = (i32, i32)> + '_ {
    // Any speed greater than the end of the target area would lead to overshooting on step 1
    (0..=*target.x.end())
        .filter(|&vx| {
            // Otherwise, we would run out of forward momentum before reaching the box
            arithmetic_progresion_from_one_to(vx) >= *target.x.start()
        })
        .flat_map(move |vx| {
            // y is always negative for the target (under the sea)
            // Any speed smaller than the start of the target area would lead to overshooting on step 1
            // Any speed larger than the start of the target + 1 would lead to overshooting on the step after we reach the sea level again since vy at that point will be -vy0 - 1
            (*target.y.start()..(-*target.y.start() + 1))
                .map(move |vy| (vx, vy))
                .filter(|&(vx, vy)| {
                    // when y becomes 0
                    let step = 2 * vy + 1;
                    let x = arithmetic_progresion_from_one_to(vx)
                        - arithmetic_progresion_from_one_to(vx - step);
                    x <= *target.x.end()
                })
                .filter(|&(vx, vy)| reaches_destination((vx, vy), target))
        })
}

// First find min x that reaches
// Then, find max y that reaches (knowing that some can overshoot)
#[aoc(day17, part1)]
fn part1(target: &TargetArea) -> i32 {
    every_initial_velocity(target)
        .map(|(_, vy)| arithmetic_progresion_from_one_to(vy))
        .max()
        .unwrap()
}

#[aoc(day17, part2)]
fn part2(target: &TargetArea) -> usize {
    every_initial_velocity(target).count()
}

#[cfg(test)]
mod tests {
    fn input() -> TargetArea {
        parse_input(include_str!("../input/2021/day17.txt"))
    }
    const EXAMPLE_INPUT_STR: &'static str = "target area: x=20..30, y=-10..-5";
    fn example_input() -> TargetArea {
        parse_input(EXAMPLE_INPUT_STR)
    }
    use super::*;
    #[test]
    fn test_trajectory() {
        assert!(reaches_destination((6, 9), &example_input()));
        assert!(reaches_destination((7, 2), &example_input()));
        assert!(reaches_destination((6, 3), &example_input()));
        assert!(reaches_destination((9, 0), &example_input()));
        assert_eq!(arithmetic_progresion_from_one_to(9), 45);
        assert_eq!(arithmetic_progresion_from_one_to(2), 3);
        assert_eq!(arithmetic_progresion_from_one_to(3), 6);
        assert_eq!(arithmetic_progresion_from_one_to(0), 0);
    }
    #[test]
    fn test_part1_given_example_input() {
        assert_eq!(part1(&example_input()), 45)
    }
    #[test]
    fn test_part1() {
        assert_eq!(part1(&input()), 4656)
    }
    #[test]
    fn test_part2_given_example_input() {
        assert_eq!(part2(&example_input()), 112)
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(&input()), 1908)
    }
}
