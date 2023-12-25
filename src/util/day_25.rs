use super::Part;


pub fn solve(input : String, part: Part) -> String {

    let lines = input.lines().collect();
    match part {
        Part::Part1 => part1(lines),
        Part::Part2 => part2(lines)
    }
}

fn part1(_lines:Vec<&str>) -> String {
    "1".to_string()
}

fn part2(_lines:Vec<&str>) -> String {
    "2".to_string()
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use util::Part::{Part1, Part2};

    const INPUT:&str = "";

    #[test]
    fn test1() {
        assert_eq!("1", solve(INPUT.to_string(), Part1));
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../../input/input_25.txt");
        assert_eq!("1", solve(input.to_string(), Part1));
    }

    #[test]
    fn test2() {
        assert_eq!("2", solve(INPUT.to_string(), Part2));
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../../input/input_25.txt");
        assert_eq!("2", solve(input.to_string(), Part2));
    }
}
