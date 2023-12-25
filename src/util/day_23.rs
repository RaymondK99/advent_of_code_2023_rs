use std::collections::{HashSet, VecDeque};
use super::Part;


pub fn solve(input : String, part: Part) -> String {

    let lines = input.lines().collect();
    match part {
        Part::Part1 => part1(lines),
        Part::Part2 => part2(lines)
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Direction {
    Left,
    Right,
    Down,
    Up,
}

impl Direction {
    fn get_delta(&self) -> (i32, i32) {
        match *self {
            Direction::Left => (-1,0),
            Direction::Right => (1,0),
            Direction::Down => (0,1),
            Direction::Up => (0,-1),
        }
    }

    fn get_next_pos(&self, pos: (i32, i32)) -> (i32, i32) {
        let (dx, dy) = self.get_delta();
        (dx + pos.0, dy + pos.1)
    }

    fn is_next_valid(&self, pos: (i32, i32), map:&Vec<Vec<char>>, part2:bool ) -> bool {
        if pos.1 == 0 && *self == Direction::Up || pos.1 == map.len() as i32 - 1 && *self == Direction::Down {
            false
        } else {
            let (next_x, next_y) = self.get_next_pos(pos);
            let next_ch = map[next_y as usize][next_x as usize];
            if part2 {
                match next_ch {
                    '#' => false,
                    _ => true,
                }
            } else {
                match next_ch {
                    '.' => true,
                    '>' => *self == Direction::Right,
                    'v' => *self == Direction::Down,
                    '<' => *self == Direction::Left,
                    '^' => *self == Direction::Up,
                    _ => false,
                }
            }

        }
    }
}

fn parse(lines:&Vec<&str>) -> Vec<Vec<char>> {
    lines.iter().map(|line| line.chars().collect()).collect()
}

fn dfs(map:&Vec<Vec<char>>, part2:bool) -> usize {
    let mut queue = VecDeque::new();
    let start = (1, 0);
    let end = (map[0].len() as i32 - 2, map.len() as i32 - 1);

    let mut max_len = 0;
    queue.push_back((HashSet::new(), start));

    while !queue.is_empty() {

        let (mut curr_visited, curr_pos) = queue.pop_front().unwrap();

        if curr_pos == end {
            // Found end
            if curr_visited.len() > max_len {
                max_len = curr_visited.len();
            }
            continue;
        } else if curr_visited.contains(&curr_pos){
            continue;
        }

        curr_visited.insert(curr_pos);

        let directions = [Direction::Left, Direction::Right, Direction::Up, Direction::Down];

        let mut next_positions = directions.iter()
            .copied()
            .filter(|dir| dir.is_next_valid(curr_pos, &map, part2))
            .map(|dir| dir.get_next_pos(curr_pos))
            .filter(|next_pos| !curr_visited.contains(next_pos))
            .collect::<Vec<(i32, i32)>>();


        if next_positions.len() == 1 {
            queue.push_front( (curr_visited, next_positions.remove(0)));
        } else {
            // Take first
            while !next_positions.is_empty() {
                queue.push_front( (curr_visited.clone(), next_positions.remove(0)));
            }
        }
    }

    max_len
}

fn part1(lines:Vec<&str>) -> String {
    let map = parse(&lines);
    dfs(&map, false).to_string()
}

fn part2(lines:Vec<&str>) -> String {
    let map = parse(&lines);
    dfs(&map, true).to_string()
}


#[cfg(test)]
mod tests {
    use std::assert_eq;
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use util::Part::{Part1, Part2};

    const INPUT:&str = "#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#";

    #[test]
    fn test1() {
        assert_eq!("94", solve(INPUT.to_string(), Part1));
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../../input/input_23.txt");
        assert_eq!("2174", solve(input.to_string(), Part1));
    }

    #[test]
    fn test2() {
        assert_eq!("154", solve(INPUT.to_string(), Part2));
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../../input/input_23.txt");
        //assert_eq!("2", solve(input.to_string(), Part2));
    }
}
