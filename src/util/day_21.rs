use std::collections::{HashSet, VecDeque};
use super::Part;


pub fn solve(input : String, part: Part) -> String {

    let lines = input.lines().collect();
    match part {
        Part::Part1 => part1(lines),
        Part::Part2 => part2(lines)
    }
}


fn parse(lines:Vec<&str>) -> Vec<Vec<char>> {
    lines.iter().map(|line| line.chars().collect()).collect()
}

fn get_start_pos(map:&Vec<Vec<char>>) -> (usize, usize) {
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] == 'S' {
                return (x,y)
            }
        }
    }

    panic!("...")
}


fn calc_steps(map:&Vec<Vec<char>>, no_steps:u32) -> usize {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    let width = map[0].len();
    let height = map.len();
    let mut visited_plots = 0;
    let (x0, y0) = get_start_pos(&map);

    queue.push_back((x0, y0, 0));

    while !queue.is_empty() {
        let item = queue.pop_front().unwrap();
        let (x, y, steps) = item;

        if map[y][x] == '#' || visited.contains(&item) || steps > no_steps {
            continue;
        } else if steps == no_steps {
            visited_plots += 1;
        }

        visited.insert(item);

        if x > 0 {
            queue.push_back((x-1,y,steps + 1));
        }
        if x < width - 1 {
            queue.push_back((x+1,y,steps + 1));
        }
        if y > 0 {
            queue.push_back((x,y-1,steps + 1));
        }

        if y < height - 1 {
            queue.push_back((x,y+1,steps + 1));
        }
    }

    visited_plots
}

fn part1(lines:Vec<&str>) -> String {
    let map = parse(lines);
    let positions = calc_steps(&map, 64);
    positions.to_string()
}

fn part2(_lines:Vec<&str>) -> String {
    "2".to_string()
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use util::Part::{Part1, Part2};

    const INPUT:&str = "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";

    #[test]
    fn test1() {
        assert_eq!("42", solve(INPUT.to_string(), Part1));
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../../input/input_21.txt");
        assert_eq!("3729", solve(input.to_string(), Part1));
    }

    #[test]
    fn test2() {
        assert_eq!("2", solve(INPUT.to_string(), Part2));
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../../input/input_21.txt");
        assert_eq!("2", solve(input.to_string(), Part2));
    }
}
