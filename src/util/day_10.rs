use std::collections::{HashSet, VecDeque};
use std::hash::Hash;
use util::day_10::Direction::{DOWN, LEFT, NotAvail, RIGHT, UP};
use super::Part;

pub fn solve(input : String, part: Part) -> String {
    let lines = input.lines().collect();
    match part {
        Part::Part1 => part1(lines),
        Part::Part2 => part2(lines)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
enum Direction {
    LEFT,
    RIGHT,
    UP,
    DOWN,
    NotAvail,
}

impl Direction {
    fn get_opposite(&self) -> Direction {
        match self {
            LEFT => RIGHT,
            RIGHT => LEFT,
            UP => DOWN,
            DOWN => UP,
            NotAvail => NotAvail,
        }
    }

    fn get_delta(&self) -> (i32,i32) {
        match *self {
            LEFT => (-1,0),
            RIGHT => (1,0),
            UP => (0,-1),
            DOWN => (0,1),
            NotAvail => panic!("..."),
        }
    }

}


#[derive(Debug, Eq, PartialEq, Hash)]
struct Tile {
    ch:char,
    entry_directions: Vec<Direction>,
    exit_directions: Vec<Direction>,
}

impl Tile {
    fn new(ch:char) -> Tile {
        let entry_directions = match ch {
            '-' => vec![LEFT,RIGHT],
            '|' => vec![UP,DOWN],
            'L' => vec![DOWN, LEFT],
            'J' => vec![DOWN, RIGHT],
            '7' => vec![UP, RIGHT],
            'F' => vec![UP, LEFT],
            'S' => vec![UP,DOWN],
            '.' => vec![NotAvail],
            _ => {
                println!("char={}", ch);
                panic!("...");
            },
        };

        let exit_directions = entry_directions.iter().map(|dir| dir.get_opposite()).collect();
        Tile{ch,entry_directions, exit_directions}
    }

    fn matches(&self, exit_dir:&Direction, other:&Tile) -> bool {
        //println!("Tile:{}, exits:{:?}, other:{}, entry:{:?}", self.ch, self.exit_directions, other.ch, other.entry_directions);
        for  entry_dir in other.entry_directions.iter() {
            if *entry_dir == NotAvail {
                break;
            } else if *exit_dir == *entry_dir {
                return true;
            }
        }

        return false;
    }

}

fn next_pos(x:i32, y:i32, map:&Vec<Vec<Tile>>) -> Vec<(i32,i32)> {
    let current_tile = map.get(y as usize).unwrap().get(x as usize).unwrap();
   let mut next_steps = vec![];


    for dir in current_tile.exit_directions.iter() {
        let (d_x, d_y) = dir.get_delta();
        let next_y = d_y + y;
        let next_x = d_x + x;

        if next_y < 0 || next_y >= map.len() as i32 || next_x < 0 || next_x >= map.first().unwrap().len() as i32 {
            continue;
        }

        let next_tile = map.get(next_y as usize).unwrap().get(next_x as usize).unwrap();

        if current_tile.matches(dir, next_tile) {
            next_steps.push((next_x, next_y));
        }
    }

    next_steps

}

fn parse_map(lines:Vec<&str>) -> Vec<Vec<Tile>> {
    lines.iter()
        .map(|row| row.chars().into_iter()
            .map(|c| Tile::new(c)).collect::<Vec<Tile>>())
        .collect()
}

fn get_pipe(map:&Vec<Vec<Tile>>) -> Vec<(i32,i32)> {
    let (start_x, start_y) = map.iter().enumerate()
        .map( |(y, row)| row.iter().enumerate()
            .map(move |(x, tile)| (x,y, tile)))
        .flatten()
        .find(|(_,_, tile)| tile.ch == 'S')
        .map(|(x,y,_)| (x as i32, y as i32))
        .unwrap();

    let mut visited = HashSet::new();
    let mut stack = VecDeque::new();
    let mut pipe = vec![];

    visited.insert((start_x, start_y));
    pipe.push((start_x, start_y));
    // Start in one direction
    let (first_step_x, first_step_y) = *next_pos(start_x, start_y, &map).first().unwrap();
    stack.push_back((1, first_step_x, first_step_y));

    // Iterate until we reach start position
    while !stack.is_empty() {
        let (steps, curr_x, curr_y) = stack.pop_front().unwrap();
        visited.insert((curr_x, curr_y));
        pipe.push((curr_x, curr_y));

        let next_steps = next_pos(curr_x, curr_y, &map);
        for (next_x, next_y) in next_steps {
            if visited.contains(&(next_x, next_y)) {
                // Already visited
                continue;
            } else {
                // Visit new node
                stack.push_back((steps+1, next_x, next_y));
            }
        }
    }

    pipe
}



fn part1(lines : Vec<&str>) -> String {

    let map = parse_map(lines);

    let pipe = get_pipe(&map);
    ((pipe.len()+1)/2).to_string()
}


fn part2(lines : Vec<&str>) -> String {
    let map = parse_map(lines);
    let mut pipe:VecDeque<(i32, i32)> = get_pipe(&map).into_iter().collect();
    let mut visited = HashSet::new();
    pipe.iter().copied().for_each( |item| {
        visited.insert(item);
    });


    let start_y = pipe.iter().map(|(_,y)| *y).min().unwrap();
    let start_x = pipe.iter().filter(|(_,y)| *y == start_y).map(|(x,_)| *x).min().unwrap();

    while *pipe.front().unwrap() != (start_x, start_y) {
        let item = pipe.pop_front().unwrap();
        pipe.push_back(item);
    }

    let mut first= true;
    let mut current_direction = Direction::DOWN;
    let mut current_inside_direction = Direction::RIGHT;
    let mut curr_x = pipe.front().unwrap().0;
    let mut curr_y = pipe.pop_front().unwrap().1;
    let mut filled_positions = HashSet::new();


    while !pipe.is_empty() {
        let (next_x,next_y) = pipe.pop_front().unwrap();
        let next_dir = if curr_x == next_x {
            if next_y < curr_y {
                UP
            } else {
                DOWN
            }
        } else if next_x < curr_x {
            LEFT
        } else {
            RIGHT
        };

        if first {
            if next_dir == DOWN {
                current_direction = DOWN;
                current_inside_direction = RIGHT;
            } else if next_dir == RIGHT {
                current_direction = RIGHT;
                current_inside_direction = DOWN;
            } else {
                panic!("unexp..");
            }
            first = false;
         }

        let mut next_inside_direction = current_inside_direction;

        // Get side to fill...
        if current_direction != next_dir {
            let turn = if current_direction == LEFT {
                if next_dir == UP {
                    RIGHT
                } else {
                    LEFT
                }
            } else if current_direction == RIGHT {
                if next_dir == DOWN {
                    RIGHT
                } else {
                    LEFT
                }
            } else if current_direction == UP {
                if next_dir == LEFT {
                    LEFT
                } else {
                    RIGHT
                }
            } else if current_direction == DOWN {
                if next_dir == LEFT {
                    RIGHT
                } else {
                    LEFT
                }
            } else {
                panic!("...");
            };

            // Update inside direction
            next_inside_direction = if turn == LEFT {
                match current_inside_direction {
                    LEFT => DOWN,
                    RIGHT => UP,
                    UP => LEFT,
                    DOWN => RIGHT,
                    NotAvail => panic!(",,"),
                }
            } else {
                match current_inside_direction {
                    LEFT => UP,
                    RIGHT => DOWN,
                    UP => RIGHT,
                    DOWN => LEFT,
                    NotAvail => panic!(",,"),
                }
            };
        }


        // Fill up inside positions
        let inside_directions = if current_inside_direction == next_inside_direction {
            vec![current_inside_direction]
        } else {
            vec![current_inside_direction, next_inside_direction]
        };

        for inside_dir in inside_directions {
            // Fill position
            let fill_pos = match inside_dir {
                UP => (curr_x, curr_y - 1),
                DOWN => (curr_x, curr_y +1),
                LEFT => (curr_x - 1, curr_y),
                RIGHT => (curr_x + 1, curr_y),
                _ => panic!("..."),
            };

            if fill_pos.0 >= 0 && fill_pos.0 < map.first().unwrap().len() as i32 && fill_pos.1 >= 0 && fill_pos.1 < map.len() as i32  {
                // try to fill and all its neightbors
                fill(fill_pos, &mut visited, &mut filled_positions);
            }

        }


        // Increment to next position
        curr_x = next_x;
        curr_y = next_y;
        current_direction = next_dir;
        current_inside_direction = next_inside_direction;

    }


    filled_positions.len().to_string()
}

fn fill(pos:(i32,i32), visited: &mut HashSet<(i32, i32)>, fill:&mut HashSet<(i32, i32)>) {
    let mut stack = VecDeque::new();
    stack.push_back(pos);

    while !stack.is_empty() {
        let item = stack.pop_front().unwrap();

        if visited.contains(&item) || fill.contains(&item){
            continue;
        }

        fill.insert(item);

        // add all adjacent nodes
        for (dx, dy) in [(0,1),(0,-1),(1,0),(-1,0)] {
            let adjacent = (item.0 + dx, item.1 + dy);
            stack.push_back(adjacent);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::assert_eq;
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use util::Part::{Part1, Part2};



    const INPUT_2:&str = "-L|F7
7S-7|
L|7||
-L-J|
L|-JF";



    const INPUT_4:&str = "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ";

    const INPUT:&str = ".....
.S-7.
.|.|.
.L-J.
.....";

    const INPUT_5:&str = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";

    const INPUT_6:&str = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";



    const INPUT_8:&str = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";

    #[test]
    fn test1() {
        assert_eq!("4", solve(INPUT_2.to_string(), Part1));
    }

    #[test]
    fn test12() {
        assert_eq!("8", solve(INPUT_4.to_string(), Part1));
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../../input/input_10.txt");
        assert_eq!("6870", solve(input.to_string(), Part1));
    }

    #[test]
    fn test2() {
        assert_eq!("1", solve(INPUT.to_string(), Part2));
    }

    #[test]
    fn test21() {
        assert_eq!("4", solve(INPUT_5.to_string(), Part2));
    }

    #[test]
    fn test22() {
        assert_eq!("8", solve(INPUT_6.to_string(), Part2));
    }

    #[test]
    fn test23() {
        assert_eq!("10", solve(INPUT_8.to_string(), Part2));
    }



    #[test]
    fn test_part2() {
        let input = include_str!("../../input/input_10.txt");
        assert_eq!("287", solve(input.to_string(), Part2));
    }
}
