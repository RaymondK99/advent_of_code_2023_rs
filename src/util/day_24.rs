use super::Part;


pub fn solve(input : String, part: Part) -> String {

    let lines = input.lines().collect();
    match part {
        Part::Part1 => part1(lines),
        Part::Part2 => part2(lines)
    }
}


#[derive(Debug)]
struct HailStone {
    x:i64,
    y:i64,
    z:i64,
    kx:i64,
    ky:i64,
    kz:i64,
}

impl HailStone {
    fn from_str(line:&str) -> HailStone {
        let mut it = line.split(|c| c == ' ' || c == ',' || c == '@')
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<i64>().unwrap());
        HailStone { x: it.next().unwrap(), y: it.next().unwrap(), z: it.next().unwrap(), kx: it.next().unwrap(), ky: it.next().unwrap(), kz: it.next().unwrap() }
    }

    fn get_x(&self, w:f64) -> f64 {
        self.x as f64 + self.kx as f64 * w
    }

    fn get_y(&self, w:f64) -> f64 {
        self.y as f64 + self.ky as f64 * w
    }

    fn calc_w_and_t(&self, other:&HailStone) -> (f64,f64) {
        let kx1 = self.kx;
        let ky1 = self.ky;
        let kx2 = other.kx;
        let ky2 = other.ky;
        let x1 = self.x;
        let x2 = other.x;
        let y1 = self.y;
        let y2 = other.y;
        let k = kx2 as f64 / kx1 as f64;
        let m = (x2 - x1) as f64 / kx1 as f64;
        //let w = ((y2 - y1) as f64 - ky1 as f64 * m) / (ky1 as f64 * k - ky2 as f64);
        let numerator = (y2 - y1) as f64 - ky1 as f64 * m;
        let denominator = ky1 as f64 * k - ky2 as f64;
        let w = numerator / denominator;
        let t = k * w+ m;
        return (w, t)
    }

    fn intersects_at_plane(&self, other:&HailStone) -> Option<(f64, f64)> {
        if self.kx == other.kx && self.ky == other.ky {
            None
        } else {
            let (w, t) = self.calc_w_and_t(other);
            //println!("w = {}, t = {}", w, t);
            if w.is_sign_positive() && w.is_finite() && t.is_sign_positive() && t.is_finite() {
                Some((other.get_x(w), other.get_y(w)))
            } else {
                None
            }
        }
    }

    fn intersects_at_plane_within(&self, other:&HailStone, min_value:f64, max_value:f64) -> bool {
        if let Some((x,y)) = self.intersects_at_plane(other) {
            let inside = x >= min_value && x <= max_value && y >= min_value && y <= max_value;
            inside
        } else {
            false
        }
    }
}

fn intersects(lines:Vec<&str>, min_value:f64, max_value:f64) -> u32 {
    let hailstones:Vec<HailStone> = lines.iter().map(|line|HailStone::from_str(line)).collect();
    let mut intersections_within_interval = 0;
    for i in 0..hailstones.len() {
        for j in i+1..hailstones.len() {
            if i != j && hailstones[i].intersects_at_plane_within(&hailstones[j], min_value, max_value) {
                //println!("{} intersects with {}", i, j);
                intersections_within_interval += 1;
            }
        }
    }
    intersections_within_interval
}
fn part1(lines:Vec<&str>) -> String {
    intersects(lines, 200000000000000.0, 400000000000000.0).to_string()
}

fn part2(_lines:Vec<&str>) -> String {
    "2".to_string()
}


#[cfg(test)]
mod tests {
    use std::assert_eq;
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use util::Part::{Part1, Part2};

    const INPUT:&str = "19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3";

    #[test]
    fn test1() {
        assert_eq!("2",     intersects(INPUT.lines().collect(), 7.0, 27.0).to_string()
        );
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../../input/input_24.txt");
        assert_eq!("27328", solve(input.to_string(), Part1));
    }

    #[test]
    fn test2() {
        assert_eq!("2", solve(INPUT.to_string(), Part2));
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../../input/input_24.txt");
        assert_eq!("2", solve(input.to_string(), Part2));
    }
}
