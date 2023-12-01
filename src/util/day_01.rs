use super::Part;

const DIGITS:[&str;10] = ["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

pub fn solve(input : String, part: Part) -> String {

    let lines = input.lines().collect();
    match part {
        Part::Part1 => part1(lines),
        Part::Part2 => part2(lines)
    }
}

fn part1(lines:Vec<&str>) -> String {
    lines.iter()
        .map(|line| 10 * find_digit(line, true, false) + find_digit(line, false, false))
        .sum::<u32>().to_string()
}

fn part2(lines:Vec<&str>) -> String {
    lines.iter()
        .map(|line| 10 * find_digit(line, true, true) + find_digit(line, false, true))
        .sum::<u32>().to_string()
}

fn find_digit(line:&str, first:bool, word:bool) -> u32 {

    let mut digit_found = 0;

    for i in 0..line.len() {
        let sub_string = &line[i..];

        let first_char = sub_string.as_bytes()[0] as char;

        if first_char.is_digit(10) {
            digit_found = first_char as u32 - '0' as u32;
        } else if word {
            for n in 0..DIGITS.len() {
                if sub_string.starts_with(DIGITS[n]) {
                    digit_found = n as u32;
                }
            }
        }

        if first && digit_found > 0 {
            return digit_found;
        } else if i == line.len() - 1 {
            return digit_found;
        }

    }

    panic!("...")
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use util::Part::{Part1, Part2};


    #[test]
    fn test1() {

        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

        assert_eq!("142", solve(input.to_string(), Part1));
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../../input/input_01.txt");

        assert_eq!("54630", solve(input.to_string(), Part1));
    }

    #[test]
    fn test2() {

        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

        assert_eq!("281", solve(input.to_string(), Part2));
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../../input/input_01.txt");

        assert_eq!("54770", solve(input.to_string(), Part2));
    }
}
