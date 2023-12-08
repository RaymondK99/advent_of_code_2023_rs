use std::collections::HashMap;
use super::Part;

pub fn solve(input : String, part: Part) -> String {
    let lines = input.lines().collect();
    match part {
        Part::Part1 => part1(lines),
        Part::Part2 => part2(lines)
    }
}


fn parse_map(mut lines: Vec<&str>) -> (Vec<char>, HashMap<&str, (&str, &str)>) {
    let instructions:Vec<char> = lines.remove(0).chars().collect();

    let mut map = HashMap::new();
    let list = lines.iter()
        .filter( |s| s.contains("="))
        .map(|line| {
            let items:Vec<&str> = line.split(|c:char| c == ' ' || c == '|' || c == ',' || c == '(' || c == ')')
                .filter(|s| s.starts_with(|c:char| c.is_alphanumeric()))
                .collect();
            items
        })
        .collect::<Vec<Vec<&str>>>();

    for item in list {
        map.insert(item[0], (item[1], item[2]));
    }

    (instructions, map)
}

fn traverse_path<F>(instructions:&Vec<char>, map:&HashMap<&str, (&str, &str)>, start_pos:&str, end_condition: F) -> usize
    where
        F: Fn(&str) -> bool,
{
    let mut pos = start_pos;
    let mut steps = 0;
    while !end_condition(pos) {
        let dir = instructions[steps % instructions.len()];
        let step = map.get(pos).unwrap();
        pos = if dir == 'L' {
            step.0
        } else {
            step.1
        };
        steps += 1;
    }

    steps
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(numbers: &[usize]) -> usize {
    let mut result = numbers[0];

    for &num in &numbers[1..] {
        result = (result * num) / gcd(result, num);
    }

    result
}

fn part1(lines : Vec<&str>) -> String {
    let (instructions, map) = parse_map(lines);
    traverse_path(&instructions, &map, "AAA", |s| s.eq("ZZZ")).to_string()
}

fn part2(lines : Vec<&str>) -> String {
    let (instructions, map) = parse_map(lines);

    let path_lengths:Vec<usize> = map.keys()
        .filter(|s| s.ends_with("A"))
        .map(|pos| traverse_path(&instructions, &map, pos, |s| s.ends_with("Z")))
        .collect();

    lcm(path_lengths.as_slice()).to_string()
}

#[cfg(test)]
mod tests {
    use std::assert_eq;
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use util::Part::{Part1, Part2};


    const INPUT:&str = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

    const INPUT2:&str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

    const INPUT3:&str  = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

    #[test]
    fn test1() {
        assert_eq!("2", solve(INPUT.to_string(), Part1));
    }

    #[test]
    fn test11() {
        assert_eq!("6", solve(INPUT2.to_string(), Part1));
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../../input/input_08.txt");
        assert_eq!("17621", solve(input.to_string(), Part1));
    }

    #[test]
    fn test2() {
        assert_eq!("6", solve(INPUT3.to_string(), Part2));
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../../input/input_08.txt");
        assert_eq!("20685524831999", solve(input.to_string(), Part2));
    }
}
