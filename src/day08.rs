use aoc_runner_derive::{aoc, aoc_generator};

/*
    0:      1:      2:      3:      4:
 aaaa    ....    aaaa    aaaa    ....
b    c  .    c  .    c  .    c  b    c
b    c  .    c  .    c  .    c  b    c
 ....    ....    dddd    dddd    dddd
e    f  .    f  e    .  .    f  .    f
e    f  .    f  e    .  .    f  .    f
 gggg    ....    gggg    gggg    ....

  5:      6:      7:      8:      9:
 aaaa    aaaa    aaaa    aaaa    aaaa
b    .  b    .  .    c  b    c  b    c
b    .  b    .  .    c  b    c  b    c
 dddd    dddd    ....    dddd    dddd
.    f  e    f  .    f  e    f  .    f
.    f  e    f  .    f  e    f  .    f
 gggg    gggg    ....    gggg    gggg";




a -> 7
b -> 6
c -> 8
d -> 7
e -> 4
f -> 9 (in all but 2)
g -> 7

b, c, e and f are unique
 * */

struct Input {
    digits: Vec<String>,
    output: Vec<String>,
}

#[aoc_generator(day08)]
fn parse_input(data: &str) -> Vec<Input> {
    data.lines()
        .map(|line| {
            let mut it = line.split(" | ").map(|segment| {
                segment
                    .split(" ")
                    .map(|word| word.to_string())
                    .collect::<Vec<_>>()
            });
            Input {
                digits: it.next().unwrap(),
                output: it.next().unwrap(),
            }
        })
        .collect()
}

#[aoc(day08, part1)]
fn part1(data: &[Input]) -> usize {
    data.iter()
        .flat_map(|line| line.output.iter())
        .filter(|word| {
            let n = word.len();
            // digit one
            n == 2 ||
            // digit 7
            n == 3 ||
            // digit 4
            n == 4 ||
            // digit 8
            n == 7
        })
        .count()
}

// digit, length
// 0 -> 6
// 1 -> 2
// 2 -> 5
// 3 -> 5
// 4 -> 4
// 5 -> 5
// 6 -> 6
// 7 -> 3
// 8 -> 7
// 9 -> 6

// letter to n occurences in digits:
// a -> 8
// b -> 6
// c -> 8
// d -> 7
// e -> 4
// f -> 9 (in all but 2)
// g -> 7

// (b, c, e and f are unique)

fn occurs_n_times(
    n_occurrences_of_digit: &[usize; 7],
    target_n: usize,
) -> impl Iterator<Item = char> + '_ {
    n_occurrences_of_digit
        .iter()
        .enumerate()
        .filter(move |(_, n)| **n == target_n)
        .map(|(index, _)| char::from_u32('a' as u32 + index as u32).unwrap())
}

// mapping:
//  a  b  c  d  e  f  g
// [0, 1, 2, 3, 4, 5, 6]
fn map_digits(input: &[String]) -> [char; 7] {
    let mut res = ['_'; 7];
    // count occurrences
    let mut n_occurrences_of_digit = [0; 7];
    for s in input {
        for c in s.chars() {
            n_occurrences_of_digit[c as usize - 'a' as usize] += 1;
        }
    }
    let one = input.iter().find(|s| s.len() == 2).unwrap();
    let seven = input.iter().find(|s| s.len() == 3).unwrap();
    let four = input.iter().find(|s| s.len() == 4).unwrap();
    // digits 1 and 7 differ by what 'a' maps to
    let a = seven
        .chars()
        .find(|x| !one.chars().any(|ref c| c == x))
        .unwrap();
    res[a as usize - 'a' as usize] = 'a';

    // b is the only one that occurs 6 times
    let b = occurs_n_times(&n_occurrences_of_digit, 6).next().unwrap();
    res[b as usize - 'a' as usize] = 'b';

    // c occurs 8 times and is not a
    let c = occurs_n_times(&n_occurrences_of_digit, 8)
        .find(|x| *x != a)
        .unwrap();
    res[c as usize - 'a' as usize] = 'c';

    // e  is the only one that occurs 4 times
    let e = occurs_n_times(&n_occurrences_of_digit, 4).next().unwrap();
    res[e as usize - 'a' as usize] = 'e';

    // f is the only one that occurs 9 times
    let f = occurs_n_times(&n_occurrences_of_digit, 9).next().unwrap();
    res[f as usize - 'a' as usize] = 'f';

    // display(4) == bcdf, so 'd' is the only char of display(4) that's not b, c or f
    let d = four.chars().find(|&x| x != b && x != c && x != f).unwrap();
    res[d as usize - 'a' as usize] = 'd';

    // the last digit we haven't seen is g
    let g_index = res.iter().enumerate().find(|(_, &c)| c == '_').unwrap().0;
    res[g_index] = 'g';
    res
}

/*
    0:      1:      2:      3:      4:
 aaaa    ....    aaaa    aaaa    ....
b    c  .    c  .    c  .    c  b    c
b    c  .    c  .    c  .    c  b    c
 ....    ....    dddd    dddd    dddd
e    f  .    f  e    .  .    f  .    f
e    f  .    f  e    .  .    f  .    f
 gggg    ....    gggg    gggg    ....

  5:      6:      7:      8:      9:
 aaaa    aaaa    aaaa    aaaa    aaaa
b    .  b    .  .    c  b    c  b    c
b    .  b    .  .    c  b    c  b    c
 dddd    dddd    ....    dddd    dddd
.    f  e    f  .    f  e    f  .    f
.    f  e    f  .    f  e    f  .    f
 gggg    gggg    ....    gggg    gggg";

 * */
fn parse_digit(display: &str) -> u8 {
    let mut sorted_chars = display.chars().collect::<Vec<_>>();
    sorted_chars.sort();
    let sorted_chars = sorted_chars.into_iter().collect::<String>();
    match sorted_chars.as_str() {
        "abcefg" => 0,
        "cf" => 1,
        "acdeg" => 2,
        "acdfg" => 3,
        "bcdf" => 4,
        "abdfg" => 5,
        "abdefg" => 6,
        "acf" => 7,
        "abcdefg" => 8,
        "abcdfg" => 9,
        _ => panic!("incorrect display string"),
    }
}

#[aoc(day08, part2)]
fn part2(data: &[Input]) -> usize {
    data.iter()
        .map(|line| {
            let digits_map = map_digits(&line.digits);
            let mut power_of_10 = 0;
            let mut n = 0;
            for mangled_output in line.output.iter().rev() {
                let demangled_output = mangled_output
                    .chars()
                    .map(|c| digits_map[c as usize - 'a' as usize])
                    .collect::<String>();
                let parsed = parse_digit(&demangled_output);
                n += parsed as usize * 10_i32.pow(power_of_10) as usize;
                power_of_10 += 1;
            }
            n
        })
        .sum()
}

#[cfg(test)]
mod tests {
    fn input() -> Vec<Input> {
        parse_input(include_str!("../input/2021/day8.txt"))
    }
    const E08AMPLE_INPUT_STR: &'static str =
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";
    fn example_input() -> Vec<Input> {
        parse_input(E08AMPLE_INPUT_STR)
    }
    use super::*;
    #[test]
    fn test_part1_given_example_input() {
        assert_eq!(part1(&example_input()), 26)
    }
    #[test]
    fn test_part1() {
        assert_eq!(part1(&input()), 445)
    }
    #[test]
    fn test_part2_given_example_input() {
        assert_eq!(part2(&example_input()), 61229)
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(&input()), 1043101)
    }
}
