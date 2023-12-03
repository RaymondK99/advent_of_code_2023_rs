use super::Part;


pub fn solve(input : String, part: Part) -> String {
    let lines = input.lines().collect();
    match part {
        Part::Part1 => part1(lines),
        Part::Part2 => part2(lines)
    }
}

struct EnginePart {
    x:usize,
    y:usize,
    part_number:u32,
}


impl EnginePart {

    fn build_parts(lines:&Vec<&str>) -> Vec<EnginePart> {
       lines.iter()
            .enumerate()
            .map(|(y,line)| parse_from_line(y, line))
            .flatten()
            .collect()
    }
    fn is_adjacent(&self, x: usize, y: usize) -> bool {
        let len = self.part_number.to_string().len();

        for xn in self.x..self.x + len {
            let xd = (x as i32 - xn as i32).abs();
            let yd = (y as i32 - self.y as i32).abs();

            if xd + yd == 1 || (xd == 1 && yd == 1) {
                return true;
            }
        }

        false
    }
}


fn parse_from_line(y:usize, line:&str) -> Vec<EnginePart> {

    let mut found_number = false;
    let mut part_number = 0;
    let mut x = 0;
    let mut parts = vec![];
    let width = line.len();

    for (xn, ch) in line.bytes().enumerate() {

        // Continue on existing number
        if found_number {
            if (ch as char).is_digit(10) {
                part_number *= 10;
                part_number += (ch - '0' as u8) as u32;
            }

            // End number
            if !(ch as char).is_digit(10 ) || xn == width - 1 {
                // Number ended
                found_number = false;
                parts.push(EnginePart{x, y, part_number});
                part_number = 0;
            }
        } else if  (ch as char).is_digit(10) {
            // Found new number
            found_number = true;
            part_number = (ch - '0' as u8) as u32;
            x = xn;
        }
    }

    parts
}

fn has_neighbor(part:&EnginePart, map:&Vec<Vec<u8>>, is_gear:bool) -> bool {

    let len = part.part_number.to_string().len();
    let w = map.first().unwrap().len() as i32;
    let h = map.len() as i32;
    let y0 = part.y as i32;
    let x0 = part.x as i32;

    // Left of part and right of part
    let mut deltas:Vec<(i32,i32)> = vec![(-1,-1),(0,-1),(1,-1), (-1,len as i32),(0,len as i32),(1,len as i32)];

    // Above and below
    for dx in 0..len {
        deltas.push((-1,dx as i32));
        deltas.push((1, dx as i32));
    }

    deltas.iter()
        .map(|(yd,xd)| (*yd+y0, *xd+x0))
        .filter(|(y,x)| *y >=0 && *x >= 0 && *y < h && *x < w)
        .map(|(y,x)| map.get(y as usize).unwrap().get(x as usize).unwrap())
        .map(|b| *b as char)
        .filter(|ch| ch.is_digit(10) || (*ch != '.' && !is_gear) || (*ch == '*' && is_gear))
        .count() > 0
}

fn part1(lines : Vec<&str>) -> String {

    let map:Vec<Vec<u8>> = lines.iter()
        .map(|line| line.bytes().collect())
        .collect();

    EnginePart::build_parts(&lines)
        .iter()
        .filter(|part| has_neighbor(&part, &map, false))
        .map(|part| part.part_number)
        .sum::<u32>()
        .to_string()
}

fn part2(lines : Vec<&str>) -> String {
    let map:Vec<Vec<u8>> = lines.iter()
        .map(|line| line.bytes().collect())
        .collect();

    let total_parts:Vec<EnginePart> = EnginePart::build_parts(&lines);
    let parts:Vec<&EnginePart> = total_parts
        .iter()
        .filter(|part| has_neighbor(part, &map, true))
        .collect();


    let mut sum = 0;
    for y in 0..map.len() {
        for x in 0..map.first().unwrap().len() {
            let ch = map[y][x] as char;
            if ch == '*' {
                // Check for adjacent parts
                let gears:Vec<&&EnginePart> = parts.iter()
                    .filter(|part| part.is_adjacent(x,y))
                    .collect();

                if gears.len() == 2 {
                    sum += gears[0].part_number * gears[1].part_number;
                }
            }
        }
    }



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

        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

        assert_eq!("4361", solve(input.to_string(), Part1));
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../../input/input_03.txt");

        assert_eq!("519444", solve(input.to_string(), Part1));
    }

    #[test]
    fn test2() {

        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

        assert_eq!("467835", solve(input.to_string(), Part2));
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../../input/input_03.txt");

        assert_eq!("74528807", solve(input.to_string(), Part2));
    }
}
