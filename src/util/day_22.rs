use std::cmp::{max, min};
use std::collections::{HashMap, HashSet, VecDeque};
use super::Part;


pub fn solve(input : String, part: Part) -> String {

    let lines = input.lines().collect();
    match part {
        Part::Part1 => part1(lines),
        Part::Part2 => part2(lines)
    }
}


#[derive(Debug,Hash, Eq, PartialEq)]
struct Point {
    x:i32,
    y:i32,
    z:i32,
}

impl Point {
    fn new(x:i32, y:i32, z:i32) -> Point {
        Point{x,y,z}
    }
}

#[derive(Debug, Hash, Eq, PartialEq)]
struct Brick {
    name:u32,
    p1:Point,
    p2:Point,
}

impl Brick {
    fn new(line:&str, name:u32) -> Brick {
        let mut it = line
            .split(|c| c == ',' || c == '~')
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<i32>().unwrap());
        let p1 = Point::new(it.next().unwrap(), it.next().unwrap(), it.next().unwrap());
        let p2 = Point::new(it.next().unwrap(), it.next().unwrap(), it.next().unwrap());
        Brick{name, p1, p2}
    }


    fn fall(&mut self, units:i32) {
        self.p1.z -= units;
        self.p2.z -= units;
    }

    fn supported_by<'a>(&'a self, bricks:&'a VecDeque<Brick>) -> Vec<&Brick> {
        let mut support = vec![];
        if self.min_z() > 1 {
            for brick in bricks.iter() {
                if self.eq(brick) {
                    continue;
                } else {
                    if self.intersects(-1, brick) {
                        support.push(brick);
                    }
                }
            }
        }
        support
    }

    fn get_fall_len(&self, bricks:&VecDeque<Brick>) -> i32 {
        let mut potential_fall = i32::MAX;
        if self.min_z() > 1  {
            for brick in bricks.iter() {
                if self.eq(brick) {
                    continue;
                } else {
                    let other_delta = self.get_potential_fall(brick);
                    //println!("Brick {} call fall {} units against brick {}", self.name, other_delta, brick.name);
                    potential_fall = min(potential_fall, other_delta);
                }
            }
        } else {
            potential_fall = 0;
        }

        potential_fall
    }


    fn min_z(&self) -> i32 {
        min(self.p1.z, self.p2.z)
    }

    fn min_x(&self) -> i32 {
        min(self.p1.x, self.p2.x)
    }

    fn min_y(&self) -> i32 {
        min(self.p1.y, self.p2.y)
    }

    fn max_z(&self) -> i32 {
        max(self.p1.z, self.p2.z)
    }

    fn max_x(&self) -> i32 {
        max(self.p1.x, self.p2.x)
    }

    fn max_y(&self) -> i32 {
        max(self.p1.y, self.p2.y)
    }

    fn intersects(&self, delta_z:i32, other:&Brick) -> bool {
        Brick::interval_intersects(self.min_x(), self.max_x(), other.min_x(), other.max_x()) &&
            Brick::interval_intersects(self.min_y(), self.max_y(), other.min_y(), other.max_y()) &&
            Brick::interval_intersects(self.min_z() + delta_z, self.max_z() + delta_z, other.min_z(), other.max_z())
    }

    fn get_potential_fall(&self, other:&Brick) -> i32 {
        if Brick::interval_intersects(self.min_x(), self.max_x(), other.min_x(), other.max_x()) &&
            Brick::interval_intersects(self.min_y(), self.max_y(), other.min_y(), other.max_y()) &&
            self.min_z() > other.max_z() - 1 {
            // We intersect AND we are above
            self.min_z() - other.max_z() - 1
        } else {
            self.min_z() - 1
        }
    }

    fn interval_intersects(s0:i32, e0:i32, s1:i32, e1:i32) -> bool {
        if s0 < s1 {
            e0 >= s1
        } else {
            e1 >= s0
        }
    }
}


fn parse_bricks(lines:Vec<&str>) -> Vec<Brick> {
    let mut i = 0;
    let mut bricks = vec![];
    for line in lines {
        bricks.push(Brick::new(line, i));
        i += 1;
    }
    bricks
}


fn arrange_bricks(lines:Vec<&str>) -> VecDeque<Brick> {
    let mut bricks = parse_bricks(lines).into_iter().collect::<Vec<Brick>>();
    bricks.sort_by( |a,b| a.min_z().cmp(&b.min_z()));
    let mut bricks = bricks.into_iter().collect::<VecDeque<Brick>>();
    let mut fell = true;

    while fell {
        fell = false;
        for _ in 0..bricks.len() {
            let mut brick = bricks.pop_front().unwrap();

            let potential_fall_len = brick.get_fall_len(&bricks);
            if potential_fall_len > 0 {
                brick.fall(potential_fall_len);
                fell = true;
            }

            bricks.push_back(brick);
        }
    }
    bricks
}

fn part1(lines:Vec<&str>) -> String {
    let bricks = arrange_bricks(lines);

    // Bricks are now arranged
    let mut next:HashSet<&Brick> = bricks.iter().collect();

    for brick in bricks.iter() {
        // Does this brick has single support?
        let support = brick.supported_by(&bricks);

        if support.len() == 1 {
            //println!("{} is only supported by {} so {} cant be removed.", brick.name, support[0].name, support[0].name);
            next.remove(support[0]);
        }
    }

    next.len().to_string()
}


fn part2(lines:Vec<&str>) -> String {
    let bricks = arrange_bricks(lines);

    let mut upstream:HashMap<&Brick, Vec<&Brick>> = HashMap::new();
    let mut downstream:HashMap<&Brick,Vec<&Brick>> = HashMap::new();

    let mut sum = 0 ;
    for brick in bricks.iter() {
        // What bricks are supporting this brick
        let support_bricks = brick.supported_by(&bricks);

        // Add do down steam map
        downstream.insert(&brick, support_bricks.iter().copied().map(|b| b).collect::<Vec<&Brick>>());

        // Add to upstream map
        for support_brick in support_bricks {
            if upstream.contains_key(support_brick) {
                upstream.get_mut(&support_brick).unwrap().push(brick);
            } else {
                upstream.insert(support_brick, vec![brick]);
            }
        }
    }

    for i in 0..bricks.len()  {
        let brick = &bricks[i];
        // check support
        let num = check_support(brick, &downstream, &upstream, &mut vec![]);
        sum += num;
    }


    sum.to_string()
}

fn check_support<'a>(brick:&'a Brick, downstream: &HashMap<&Brick, Vec<&Brick>>, upstream: &HashMap<&Brick, Vec<&'a Brick>>, acc:&mut Vec<&'a Brick>) -> u32{
    let mut sum = 0;
    acc.push(brick);

    let item = upstream.get(&brick);
    if item.is_some() {
        let list = item.unwrap();
        for brick_above in list.iter() {
            if !acc.contains(brick_above) && !has_support(*brick_above, downstream, acc) {
                sum += 1 + check_support(*brick_above, downstream, upstream, acc)
            }
        }
    }

    sum
}


fn has_support(brick:&Brick, downstream: &HashMap<&Brick, Vec<&Brick>>, acc:&Vec<&Brick>) -> bool {
    if brick.min_z() == 1 {
        return true;
    } else {
        let support_bricks = downstream.get(brick).unwrap().iter()
            .filter(|c| !acc.contains(c))
            .copied()
            .collect::<Vec<&Brick>>();

        support_bricks.iter().any(|support| has_support(*support, downstream, acc))
    }
}

#[cfg(test)]
mod tests {
    use std::assert_eq;
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use util::Part::{Part1, Part2};

    const INPUT:&str = "1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9";

    #[test]
    fn test1() {
        assert_eq!("5", solve(INPUT.to_string(), Part1));
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../../input/input_22.txt");
        assert_eq!("451", solve(input.to_string(), Part1));
    }

    #[test]
    fn test2() {
        assert_eq!("7", solve(INPUT.to_string(), Part2));
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../../input/input_22.txt");
        assert_eq!("66530", solve(input.to_string(), Part2));
    }
}
