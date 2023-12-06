use super::Part;

pub fn solve(input : String, part: Part) -> String {
    let lines = input.lines().collect();
    match part {
        Part::Part1 => part1(lines),
        Part::Part2 => part2(lines)
    }
}

struct Race {
    time:u64,
    distance:u64,
}

impl Race {
    fn race(&self) -> usize {
        (1..self.time).into_iter()
            .map(|hold| (self.time - hold) * hold)
            .filter(|dist| *dist > self.distance)
            .count()
    }
}

fn parse(lines: Vec<&str>) -> Vec<Race> {
    let mut races = vec![];
    let numbers:Vec<Vec<u64>> = lines.iter()
        .map(|s| s.split(':').last().unwrap())
        .map(|s| s.split(' ')
            .filter(|s| s.starts_with(|c:char| c.is_digit(10)))
            .map(|s| s.parse::<u64>().unwrap())
            .collect::<Vec<u64>>())
        .collect();

    for i in 0..numbers.first().unwrap().len() {
        races.push(Race{time:numbers[0][i], distance:numbers[1][i]});
    }
    races
}


fn part1(lines : Vec<&str>) -> String {
    let races = parse(lines);
    races.iter().map(|race| race.race()).product::<usize>().to_string()
}

fn part2(lines : Vec<&str>) -> String {
    let next:Vec<String> = lines.iter().map(|line| line.replace(" ", "")).collect();
    let races = parse(next.iter().map(|s|s.as_str()).collect());
    races.iter().map(|race| race.race()).product::<usize>().to_string()
}


#[cfg(test)]
mod tests {
    use std::assert_eq;
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use util::Part::{Part1, Part2};


    const INPUT:&str = "Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn test1() {
        assert_eq!("288", solve(INPUT.to_string(), Part1));
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../../input/input_06.txt");
        assert_eq!("781200", solve(input.to_string(), Part1));
    }

    #[test]
    fn test2() {
        assert_eq!("71503", solve(INPUT.to_string(), Part2));
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../../input/input_06.txt");
        assert_eq!("49240091", solve(input.to_string(), Part2));
    }
}
