use std::collections::{HashSet, VecDeque};
use std::fmt::Debug;
use std::hash::Hash;
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
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn is_vertical(&self) -> bool {
        self == &Direction::Up || self == &Direction::Down
    }
    fn is_horizontal(&self) -> bool {
        !self.is_vertical()
    }
}


#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Pos {
    x:i32,
    y:i32,
}

impl Pos {
    fn new(x:i32, y:i32) -> Pos {
        Pos{x,y}
    }

    fn next(&self, dir:Direction) -> Pos {
        match dir {
            Direction::Up => Pos::new(self.x, self.y - 1),
            Direction::Down => Pos::new(self.x, self.y + 1),
            Direction::Left => Pos::new(self.x - 1, self.y),
            Direction::Right => Pos::new(self.x + 1, self.y),
        }
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct Beam {
    current:Pos,
    direction:Direction,
}

impl Beam {
    fn start(current:Pos, direction:Direction) -> Beam {
        Beam{current, direction}
    }

    fn step(&mut self, dir:Direction) {
        let next = self.current.next(dir);
        self.direction = dir;
        self.current = next;
    }

    fn is_outside(&self, map:&Vec<Vec<char>>) -> bool {
        self.current.x < 0 || self.current.y < 0 || self.current.y >= map.len() as i32 || self.current.x >= map[0].len() as i32
    }

}

fn parse(lines : Vec<&str>) -> Vec<Vec<char>> {
    lines.iter().map(|line| line.chars().collect()).collect()
}


fn resolve_beam(map:&Vec<Vec<char>>, start_beam:Beam) -> usize {
    let mut beams = VecDeque::from(vec![start_beam]);
    let mut beam_paths = HashSet::new();
    let mut visited = HashSet::new();

    while !beams.is_empty() {
        let mut beam = beams.pop_front().unwrap();

        if beam.is_outside(&map) || beam_paths.contains(&beam) {
            // drop beam
            continue;
        } else {
            beam_paths.insert(beam.clone());
            visited.insert(beam.current);
        }

        // move beam
        let curr_pos = beam.current;
        let ch = map[curr_pos.y as usize][curr_pos.x as usize];

        if ch == '.' || (beam.direction.is_horizontal() && ch == '-') || (beam.direction.is_vertical() && ch == '|'){
            // Continue in same direction
            beam.step(beam.direction);
        }  else if ch == '\\' && beam.direction == Direction::Right {
            beam.step(Direction::Down);
        } else if ch == '\\' && beam.direction == Direction::Down {
            beam.step(Direction::Right);
        } else if ch == '\\' && beam.direction == Direction::Up {
            beam.step(Direction::Left);
        } else if ch == '\\' && beam.direction == Direction::Left {
            beam.step(Direction::Up);
        } else if ch == '/' && beam.direction == Direction::Right {
            beam.step(Direction::Up);
        } else if ch == '/' && beam.direction == Direction::Left {
            beam.step(Direction::Down);
        } else if ch == '/' && beam.direction == Direction::Up {
            beam.step(Direction::Right);
        } else if ch == '/' && beam.direction == Direction::Down {
            beam.step(Direction::Left);
        } else if ch == '|' && beam.direction.is_horizontal() {
            // Split up and down
            let mut beam_down = beam.clone();
            beam_down.step(Direction::Down);
            beams.push_back(beam_down);
            beam.step(Direction::Up);
        } else if ch == '-' && beam.direction.is_vertical() {
            // Split left and right
            let mut beam_right = beam.clone();
            beam_right.step(Direction::Right);
            beams.push_back(beam_right);
            beam.step(Direction::Left)
        }

        beams.push_back(beam);
    }

    visited.len()
}

fn part1(lines : Vec<&str>) -> String {
    let map = parse(lines);
    resolve_beam(&map, Beam::start(Pos::new(0,0), Direction::Right)).to_string()
}

fn part2(lines : Vec<&str>) -> String {
    let map = parse(lines);
    let mut beams = vec![];

    for n in 0..map.len() {
        beams.push(Beam::start(Pos::new(0, n as i32), Direction::Right));
        beams.push(Beam::start(Pos::new(map[0].len() as i32 - 1, n as i32), Direction::Left));
        beams.push(Beam::start(Pos::new(n as i32, 0), Direction::Down));
        beams.push(Beam::start(Pos::new(n as i32, map.len() as i32 - 1), Direction::Up));
    }

    beams.iter()
        .map(|beam| resolve_beam(&map, beam.clone()))
        .max()
        .unwrap()
        .to_string()
}


#[cfg(test)]
mod tests {
    use std::assert_eq;
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use util::Part::{Part1, Part2};


    const INPUT:&str = r#".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."#;

    #[test]
    fn test1() {
        assert_eq!("46", solve(INPUT.to_string(), Part1));
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../../input/input_16.txt");
        assert_eq!("7415", solve(input.to_string(), Part1));
    }

    #[test]
    fn test2() {
        assert_eq!("51", solve(INPUT.to_string(), Part2));
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../../input/input_16.txt");
        assert_eq!("7943", solve(input.to_string(), Part2));
    }
}
