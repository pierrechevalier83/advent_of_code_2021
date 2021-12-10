use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day10)]
fn parse_input(data: &str) -> Vec<String> {
    data.lines().map(|s| s.to_owned()).collect::<Vec<_>>()
}

fn matching_closing_brace(c: char) -> char {
    match c {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => unreachable!("unexpected closing brace"),
    }
}

enum SyntaxError {
    Corrupted(char),
    Incomplete(Vec<char>),
}

fn error_score(e: SyntaxError) -> usize {
    match e {
        SyntaxError::Corrupted(')') => 3,
        SyntaxError::Corrupted(']') => 57,
        SyntaxError::Corrupted('}') => 1197,
        SyntaxError::Corrupted('>') => 25137,
        SyntaxError::Incomplete(stack) => stack
            .iter()
            .rev()
            .copied()
            .map(matching_closing_brace)
            .map(|c| match c {
                ')' => 1,
                ']' => 2,
                '}' => 3,
                '>' => 4,
                _ => panic!("unexpected completion error"),
            })
            .fold(0, |acc, score| 5 * acc + score),
        _ => unreachable!("unexpected syntax error"),
    }
}

fn check_syntax(data: &[String]) -> impl Iterator<Item = SyntaxError> + '_ {
    data.iter().filter_map(|line| {
        let mut stack = Vec::with_capacity(line.len());
        for c in line.chars() {
            match c {
                '(' | '[' | '{' | '<' => stack.push(c),
                close => {
                    let open = stack.pop();
                    let expected = open.map(|o| matching_closing_brace(o));
                    let got = close;
                    if expected != Some(got) {
                        return Some(SyntaxError::Corrupted(got));
                    }
                }
            }
        }
        if !stack.is_empty() {
            return Some(SyntaxError::Incomplete(stack));
        }
        None
    })
}

#[aoc(day10, part1)]
fn part1(data: &[String]) -> usize {
    check_syntax(data)
        .filter(|e| match e {
            SyntaxError::Corrupted(_) => true,
            _ => false,
        })
        .map(error_score)
        .sum()
}

#[aoc(day10, part2)]
fn part2(data: &[String]) -> usize {
    let mut completion_scores = check_syntax(data)
        .filter(|e| match e {
            SyntaxError::Incomplete(_) => true,
            _ => false,
        })
        .map(error_score)
        .collect::<Vec<_>>();
    completion_scores.sort_unstable();
    completion_scores[completion_scores.len() / 2]
}

#[cfg(test)]
mod tests {
    fn input() -> Vec<String> {
        parse_input(include_str!("../input/2021/day10.txt"))
    }
    const EXAMPLE_INPUT_STR: &'static str = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]
";
    fn example_input() -> Vec<String> {
        parse_input(EXAMPLE_INPUT_STR)
    }
    use super::*;
    #[test]
    fn test_part1_given_example_input() {
        assert_eq!(part1(&example_input()), 26397)
    }
    #[test]
    fn test_part1() {
        assert_eq!(part1(&input()), 413733)
    }
    #[test]
    fn test_part2_given_example_input() {
        assert_eq!(part2(&example_input()), 288957)
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(&input()), 3354640192)
    }
}
