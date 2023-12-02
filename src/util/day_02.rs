use util::day_02::Color::{Blue, Green, Red};
use super::Part;

pub fn solve(input : String, part: Part) -> String {
    let lines = input.lines().collect();
    match part {
        Part::Part1 => part1(lines),
        Part::Part2 => part2(lines)
    }
}

#[derive(Debug, PartialEq)]
enum Color {
    Blue,
    Red,
    Green,
}
#[derive(Debug)]
struct Cubes {
    color:Color,
    num:usize
}

fn parse_line(line:&str)  -> Vec<Vec<Cubes>> {
    line.
        split(|c| c == ':' || c == ';')
        .map(|s| s.trim())
        .filter(|s| !s.contains("Game"))
        .map(|hand| {
            hand.split(',').map(|h| h.trim())
                .map(|cubes| {
                    let items:Vec<&str> = cubes.split(' ').collect();
                    let num:usize = items[0].parse().unwrap();
                    let color = match items[1] {
                        "red" => Red,
                        "blue" => Blue,
                        "green" => Green,
                        _ => panic!("...")
                    };
                    Cubes{num, color}
                })
                .collect()
        })
        .collect()
}

fn game_possible(game:&Vec<Vec<Cubes>>) -> bool {
    game.iter().flatten()
        .into_iter()
        .fold(true, |a , cube| {
            let game_possible = match cube.color {
                Red => cube.num <= 12,
                Blue => cube.num <= 14,
                Green => cube.num <= 13,
            };
            a && game_possible
        })
}


fn min_num_cubes(game:&Vec<Vec<Cubes>>) -> usize {
    let cubes:Vec<&Cubes> = game.into_iter().flatten().collect();

    let reds = cubes.iter().filter(|c| c.color == Red).map(|c| c.num).max().unwrap();
    let blues = cubes.iter().filter(|c| c.color == Blue).map(|c| c.num).max().unwrap();
    let greens = cubes.iter().filter(|c| c.color == Green).map(|c| c.num).max().unwrap();

    reds * blues * greens
}


fn part1(lines: Vec<&str>) -> String {

    let sum:usize = lines.iter()
        .map(|line| parse_line(line))
        .enumerate()
        .filter(|(_, game)| game_possible(&game))
        .map(|(game_no,_)| game_no+1)
        .sum();

    sum.to_string()
}

fn part2(lines : Vec<&str>) -> String {
    let sum:usize = lines.iter()
        .map(|line| parse_line(line))
        .map(|game| min_num_cubes(&game))
        .sum();

    sum.to_string()
}


#[cfg(test)]
mod tests {
    use std::assert_eq;
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use util::Part::{Part1, Part2};


    #[test]
    fn test1() {

        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        assert_eq!("8", solve(input.to_string(), Part1));
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../../input/input_02.txt");

        assert_eq!("2239", solve(input.to_string(), Part1));
    }

    #[test]
    fn test2() {

        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        assert_eq!("2286", solve(input.to_string(), Part2));
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../../input/input_02.txt");

        assert_eq!("83435", solve(input.to_string(), Part2));
    }
}
