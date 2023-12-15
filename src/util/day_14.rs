use super::Part;

pub fn solve(input : String, part: Part) -> String {
    let lines = input.lines().collect();
    match part {
        Part::Part1 => part1(lines),
        Part::Part2 => part2(lines)
    }
}


fn parse_platform(lines : Vec<&str>) -> Vec<Vec<char>> {
    lines.iter().map(|line| line.chars().collect::<Vec<char>>()).collect()
}

fn move_rocks_horizontal(platform:&mut Vec<Vec<char>>, left:bool) {
    for y in 0..platform.first().unwrap().len() {
        loop {
            let mut swap = false;
            for x in 1..platform.len() {
                if left && platform[y][x] == 'O' && platform[y][x - 1] == '.' {
                    // swap
                    platform[y][x] = '.';
                    platform[y][x - 1] = 'O';
                    swap = true;
                } else if !left && platform[y][x] == '.' && platform[y][x - 1] == 'O' {
                    // swap
                    platform[y][x] = 'O';
                    platform[y][x - 1] = '.';
                    swap = true;
                }
            }

            if swap == false {
                break;
            }
        }
    }
}

fn move_rocks_vertical(platform:&mut Vec<Vec<char>>, up:bool) {
    for x in 0..platform.first().unwrap().len() {
        loop {
            let mut swap = false;
            for y in 1..platform.len() {
                if up && platform[y][x] == 'O' && platform[y - 1][x] == '.' {
                    // swap
                    platform[y][x] = '.';
                    platform[y - 1][x] = 'O';
                    swap = true;
                } else if !up && platform[y][x] == '.' && platform[y - 1][x] == 'O' {
                    // swap
                    platform[y][x] = 'O';
                    platform[y - 1][x] = '.';
                    swap = true;
                }
            }

            if swap == false {
                break;
            }
        }
    }
}

fn calc_weight(platform:&Vec<Vec<char>>) -> usize {
    platform.iter()
        .enumerate()
        .map(|(y,row)| row.iter()
            .filter(|item| **item == 'O')
            .map(|_| platform.len() - y)
            .sum::<usize>())
        .sum::<usize>()
}



fn part1(lines : Vec<&str>) -> String {
    let mut platform = parse_platform(lines);
    move_rocks_vertical(&mut platform, true);
    calc_weight(&platform).to_string()
}

fn tilt(platform:&mut Vec<Vec<char>>) {
    move_rocks_vertical(platform, true);
    move_rocks_horizontal(platform, true);
    move_rocks_vertical(platform, false);
    move_rocks_horizontal(platform, false);
}

fn check_cycle(numbers:&Vec<usize>) -> Option<usize> {
    for cycle_len in 4..numbers.len() / 2 {
        let mut matched = true;
        for n in 0..cycle_len {
            let last_cycle = numbers.len() - 1 - n;
            let prev_cycle = numbers.len() - 1 - n - cycle_len;
            matched = matched && numbers[last_cycle] == numbers[prev_cycle];
        }

        if matched {
            return Some(cycle_len);
        }
    }

    None
}
fn part2(lines : Vec<&str>) -> String {
    let mut platform = parse_platform(lines);
    let mut weights = vec![];
    loop {
        tilt(&mut platform);
        weights.push(calc_weight(&platform));
        match check_cycle(&weights) {
            Some(cycle_len) => {
                let rest = (1000000000 - weights.len()) % cycle_len;
                for _ in 0..rest {
                    tilt(&mut platform);
                }

                return calc_weight(&platform).to_string()
            },
            _ => {},
        }
    }
}


#[cfg(test)]
mod tests {
    use std::assert_eq;
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use util::Part::{Part1, Part2};


    const INPUT:&str = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

    #[test]
    fn test1() {
        assert_eq!("136", solve(INPUT.to_string(), Part1));
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../../input/input_14.txt");
        assert_eq!("109098", solve(input.to_string(), Part1));
    }

    #[test]
    fn test2() {
        assert_eq!("64", solve(INPUT.to_string(), Part2));
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../../input/input_14.txt");
        assert_eq!("100064", solve(input.to_string(), Part2));
    }
}
