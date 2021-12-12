use aoc_runner_derive::{aoc, aoc_generator};
use bit_set::BitSet;
use std::str::FromStr;

const START: &'static str = "start";
const END: &'static str = "end";

struct Graph {
    connections: Vec<Vec<usize>>,
    start: usize,
    end: usize,
    first_lowercase: usize,
}

impl FromStr for Graph {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut alphabetical_caves = s
            .lines()
            .flat_map(|line| line.split('-').map(|s| s.to_owned()))
            .collect::<Vec<_>>();
        alphabetical_caves.sort();
        alphabetical_caves.dedup();
        let start = alphabetical_caves
            .binary_search(&START.to_string())
            .unwrap();
        let end = alphabetical_caves.binary_search(&END.to_string()).unwrap();
        let first_lowercase = alphabetical_caves
            .iter()
            .position(|s| s.chars().next().unwrap().is_lowercase())
            .unwrap();
        let mut connections = Vec::new();
        connections.resize(s.lines().count(), Vec::new());
        for connection in s.lines() {
            let (l, r) = connection.split_once('-').ok_or("Unexpected line")?;
            let l_index = alphabetical_caves.binary_search(&l.to_string()).unwrap();
            let r_index = alphabetical_caves.binary_search(&r.to_string()).unwrap();
            connections[l_index].push(r_index);
            connections[r_index].push(l_index);
        }
        Ok(Self {
            connections,
            start,
            end,
            first_lowercase,
        })
    }
}

impl Graph {
    fn is_large(&self, cave: usize) -> bool {
        cave < self.first_lowercase
    }
    fn is_edge_node(&self, cave: usize) -> bool {
        cave == self.start || cave == self.end
    }
    fn is_small(&self, cave: usize) -> bool {
        !self.is_large(cave) && !self.is_edge_node(cave)
    }

    /// all_paths: a Map from a node to all nodes one closer to destination
    fn count_paths_to_end<'a>(
        &'a self,
        start: usize,
        path: &mut BitSet,
        can_visit_one_small_cave_twice: bool,
    ) -> usize {
        path.insert(start);
        if start == self.end {
            1
        } else {
            self.connections[start]
                .iter()
                .map(|cave| {
                    let mut seen_twice = false;
                    if self.is_large(*cave)
                        || !path.contains(*cave)
                        || can_visit_one_small_cave_twice && self.is_small(*cave) && {
                            seen_twice = true;
                            true
                        }
                    {
                        self.count_paths_to_end(
                            *cave,
                            &mut path.clone(),
                            can_visit_one_small_cave_twice && !seen_twice,
                        )
                    } else {
                        0
                    }
                })
                .sum()
        }
    }
    fn num_paths_to_end(&self, can_visit_one_small_cave_twice: bool) -> usize {
        let mut path = BitSet::new();
        self.count_paths_to_end(self.start, &mut path, can_visit_one_small_cave_twice)
    }
}

#[aoc_generator(day12)]
fn parse_input(data: &str) -> Graph {
    data.parse().unwrap()
}

#[aoc(day12, part1)]
fn part1(graph: &Graph) -> usize {
    graph.num_paths_to_end(false)
}

#[aoc(day12, part2)]
fn part2(graph: &Graph) -> usize {
    graph.num_paths_to_end(true)
}

#[cfg(test)]
mod tests {
    fn input() -> Graph {
        parse_input(include_str!("../input/2021/day12.txt"))
    }
    const SMALL_EXAMPLE_INPUT_STR: &'static str = "start-A
start-b
A-c
A-b
b-d
A-end
b-end";
    const MEDIUM_EXAMPLE_INPUT_STR: &'static str = "dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc";
    const LARGE_EXAMPLE_INPUT_STR: &'static str = "fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW";
    use super::*;
    #[test]
    fn test_part1_given_example_input() {
        assert_eq!(part1(&parse_input(SMALL_EXAMPLE_INPUT_STR)), 10);
        assert_eq!(part1(&parse_input(MEDIUM_EXAMPLE_INPUT_STR)), 19);
        assert_eq!(part1(&parse_input(LARGE_EXAMPLE_INPUT_STR)), 226)
    }
    #[test]
    fn test_part1() {
        assert_eq!(part1(&input()), 5252)
    }
    #[test]
    fn test_part2_given_example_input() {
        assert_eq!(part2(&parse_input(SMALL_EXAMPLE_INPUT_STR)), 36);
        assert_eq!(part2(&parse_input(MEDIUM_EXAMPLE_INPUT_STR)), 103);
        assert_eq!(part2(&parse_input(LARGE_EXAMPLE_INPUT_STR)), 3509);
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(&input()), 147784)
    }
}
