use super::Part;

pub fn solve(input : String, part: Part) -> String {
    let lines = input.lines().collect();
    match part {
        Part::Part1 => part1(lines),
        Part::Part2 => part2(lines)
    }
}

enum Direction {
    Up,
    Down,
    Left,
    Right
}

impl Direction {
    fn new(s:&str) -> Direction {
        match s.chars().next().unwrap() {
            'U' => Direction::Up,
            'D' => Direction::Down,
            'L' => Direction::Left,
            'R' => Direction::Right,
            '0' => Direction::Right,
            '1' => Direction::Down,
            '2' => Direction::Left,
            '3' => Direction::Up,
            _ => panic!("..."),
        }
    }
}

fn parse_line(line:&str, part2: bool) -> (Direction, i64) {
    let mut it = line.split(' ');
    if part2 {
        let hex_str = &it.last().unwrap()[2..8];
        let ch = &hex_str[5..];
        let dir = Direction::new(ch);
        let number = u64::from_str_radix(&hex_str[0..5], 16).unwrap() as i64;
        (dir, number)
    } else {
        let dir = Direction::new(it.next().unwrap());
        let len = it.next().unwrap().parse::<i64>().unwrap();
        (dir, len)
    }
}


fn build_map(lines : Vec<&str>, part2:bool) -> Vec<(i64,i64)> {
    let mut points = vec![];
    points.push((0,0));
    lines.iter().map(|line| parse_line(line, part2))
        .for_each(|(dir, len)| {
            let (x,y) = *points.last().unwrap();
            let next_pos = match dir {
                Direction::Up => (x, y - len),
                Direction::Down => (x, y + len),
                Direction::Left => (x - len, y),
                Direction::Right => (x + len, y),
            };
            points.push(next_pos);
        });
    points
}



fn shoelace_formula(points:&Vec<(i64,i64)>) -> i64 {
    let mut area = 0;
    let mut length = 0;

    for i in 0..points.len() - 1 {
        let (x1,y1) = points[i];
        let (x2,y2) = points[i+1];
        area += x1 * y2 - y1 * x2;
        length += (x1 - x2).abs() + (y1-y2).abs();
    }

    area/2 + ((length + 2) / 2)
}


fn part1(lines : Vec<&str>) -> String {
    let points = build_map(lines, false);
    shoelace_formula(&points).to_string()
}

fn part2(lines : Vec<&str>) -> String {
    let points = build_map(lines, true);
    shoelace_formula(&points).to_string()
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use util::Part::{Part1, Part2};


    const INPUT:&str = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";



    #[test]
    fn test1() {
        assert_eq!("62", solve(INPUT.to_string(), Part1));
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../../input/input_18.txt");
        assert_eq!("38188", solve(input.to_string(), Part1));
    }

    #[test]
    fn test2() {
        assert_eq!("952408144115", solve(INPUT.to_string(), Part2));
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../../input/input_18.txt");
        assert_eq!("93325849869340", solve(input.to_string(), Part2));
    }
}
