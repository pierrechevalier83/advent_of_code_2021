use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;
use std::str::FromStr;

const START: &'static str = "start";
const END: &'static str = "end";

struct Graph {
    connections: HashMap<String, Vec<String>>,
}

impl FromStr for Graph {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut connections = HashMap::new();
        for connection in s.lines() {
            let (l, r) = connection.split_once('-').ok_or("Unexpected line")?;
            connections
                .entry(l.to_string())
                .or_insert(vec![])
                .push(r.to_string());
            connections
                .entry(r.to_string())
                .or_insert(vec![])
                .push(l.to_string());
        }
        Ok(Self { connections })
    }
}

fn is_large(cave: &str) -> bool {
    cave.chars().next().unwrap().is_uppercase()
}
fn is_edge_node(cave: &str) -> bool {
    cave == START || cave == END
}
fn is_small(cave: &str) -> bool {
    !is_large(cave) && !is_edge_node(cave)
}

impl Graph {
    /// all_paths: a Map from a node to all nodes one closer to destination
    fn all_paths_to_end<'a>(
        &'a self,
        start: &'a str,
        path: &mut Vec<&'a str>,
        can_visit_one_small_cave_twice: bool,
    ) -> Vec<Vec<&'a str>> {
        path.push(start);
        if start == END {
            vec![path.clone()]
        } else {
            self.connections[start]
                .iter()
                .flat_map(|cave| {
                    let mut seen_twice = false;
                    if is_large(cave)
                        || !path.contains(&cave.as_str())
                        || can_visit_one_small_cave_twice && is_small(cave) && {
                            seen_twice = true;
                            true
                        }
                    {
                        self.all_paths_to_end(
                            cave,
                            &mut path.clone(),
                            can_visit_one_small_cave_twice && !seen_twice,
                        )
                    } else {
                        vec![]
                    }
                })
                .collect::<Vec<_>>()
        }
    }
    fn num_paths_to_end(&self, can_visit_one_small_cave_twice: bool) -> usize {
        let mut path = vec![];
        let all = self.all_paths_to_end(START, &mut path, can_visit_one_small_cave_twice);
        all.len()
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
