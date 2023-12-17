use std::cmp::{Ordering};
use std::collections::{BinaryHeap, HashMap};
use super::Part;

pub fn solve(input : String, part: Part) -> String {
    let lines = input.lines().collect();
    match part {
        Part::Part1 => part1(lines),
        Part::Part2 => part2(lines)
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
enum Direction {
    None,
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Copy, Clone)]
struct Step {
    heat_loss:u32,
    x:usize,
    y:usize,
    direction:Direction,
    consecutive_steps:u8,
}

impl Eq for Step {}

impl PartialEq<Self> for Step {
    fn eq(&self, other: &Self) -> bool {
        self.heat_loss == other.heat_loss && self.x == other.x && self.y == other.y
    }
}

impl PartialOrd<Self> for Step {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {

        let dist = self.y + self.x;
        let other_dist = other.y + self.x;

        if self.heat_loss == other.heat_loss {
            // Compare distance
            Some(dist.cmp(&other_dist))
        } else {
            Some(other.heat_loss.cmp(&self.heat_loss))
        }
    }
}

impl Ord for Step {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Step {

    fn get_next_steps(&self, map:&Vec<Vec<u32>>) -> Vec<Step> {
        let w = map[0].len();
        let h = map.len();

        let mut steps = vec![];
        if self.direction != Direction::Down &&  self.y > 0 {
            steps.push(self.next_step(Direction::Up, map));
        }

        if self.direction != Direction::Up &&  self.y != h - 1 {
            steps.push(self.next_step(Direction::Down, map));
        }

        if self.direction != Direction::Left &&  self.x != w - 1 {
            steps.push(self.next_step(Direction::Right, map));
        }

        if self.direction != Direction::Right &&  self.x > 0 {
            steps.push(self.next_step(Direction::Left, map));
        }

        steps
    }
    fn next_step(&self, direction:Direction, map:&Vec<Vec<u32>>) -> Step {
        let (x,y) = match direction {
            Direction::Up => (self.x, self.y - 1),
            Direction::Down => (self.x, self.y + 1),
            Direction::Left => (self.x - 1, self.y),
            Direction::Right => (self.x + 1, self.y),
            _ => panic!("..."),
        };
        let heat_loss_pos = map[y][x];
        let heat_loss = self.heat_loss + heat_loss_pos;
        let consecutive_steps = if self.direction == direction {
            self.consecutive_steps + 1
        } else {
            1
        };

        Step{heat_loss, x, y, direction, consecutive_steps}
    }
}

fn parse(lines : Vec<&str>) -> Vec<Vec<u32>> {
    lines.iter().map(|line| line.chars().map(|ch| ch.to_digit(10).unwrap()).collect()).collect()
}

fn calc_min_heat_loss(lines : Vec<&str>, min_steps:u8, max_steps:u8) -> u32 {
    let map = parse(lines);
    let first_step = Step{ heat_loss: 0, x: 0, y: 0, direction: Direction::None, consecutive_steps: 0};
    let height = map.len();
    let width = map[0].len();

    let mut queue: BinaryHeap<Step> = BinaryHeap::new();
    let mut visited = HashMap::new();
    let mut i = 0;

    // Insert first step
    queue.push(first_step);


    while !queue.is_empty() {
        i += 1;
        let step = queue.pop().unwrap();

        let max_heat = (step.x + step.y) * 9;
        if step.heat_loss > max_heat as u32 {
            // Useless step
            continue;
        }

        // Add as visited
        let prev_element_opt = visited.get(&(step.x, step.y, step.direction));

        if prev_element_opt.is_some() {
            let prev_step:&Step = prev_element_opt.unwrap();
            if step.consecutive_steps >= min_steps && prev_step.consecutive_steps >= min_steps && prev_step.heat_loss <= step.heat_loss && prev_step.consecutive_steps <= step.consecutive_steps {
                continue;
            }
        }

        visited.insert((step.x, step.y, step.direction), step.clone());

        // Did we reach finish?
        if step.y == height - 1 && step.x == width - 1 && step.consecutive_steps >= min_steps {
            println!("Reached finish line with heat:{}, after {} iterations", step.heat_loss, i);
            return step.heat_loss;
        }

        // Get next steps
        step.get_next_steps(&map).into_iter().for_each(|next| {
            let min_step_ok = step.direction == Direction::None || (step.direction != next.direction && step.consecutive_steps >= min_steps) || step.direction == next.direction;
            let max_step_ok = next.consecutive_steps <= max_steps;

            if max_step_ok && min_step_ok {
                queue.push(next);
            }
        })
    }

    // No solution
    return 0;
}

fn part1(lines : Vec<&str>) -> String {
    let res = calc_min_heat_loss(lines,0, 3);
    res.to_string()
}

fn part2(lines : Vec<&str>) -> String {
    let res = calc_min_heat_loss(lines,4, 10);
    res.to_string()
}


#[cfg(test)]
mod tests {
    use std::assert_eq;
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use util::Part::{Part1, Part2};


    const INPUT:&str = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";

    const INPUT2:&str = "111111111111
999999999991
999999999991
999999999991
999999999991";

    #[test]
    fn test1() {
        assert_eq!("102", solve(INPUT.to_string(), Part1));
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../../input/input_17.txt");
        assert_eq!("698", solve(input.to_string(), Part1));
    }

    #[test]
    fn test2() {
        assert_eq!("94", solve(INPUT.to_string(), Part2));
        assert_eq!("71", solve(INPUT2.to_string(), Part2));

    }

    #[test]
    fn test_part2() {

        /*
        let input = include_str!("../../input/input_17.txt");
        assert_eq!("825", solve(input.to_string(), Part2));
         */
    }
}
