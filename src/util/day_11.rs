use std::cmp::{max, min};
use super::Part;

pub fn solve(input : String, part: Part) -> String {
    let lines = input.lines().collect();
    match part {
        Part::Part1 => part1(lines),
        Part::Part2 => part2(lines)
    }
}

fn parse(lines : Vec<&str>) -> Vec<Vec<char>> {
    let galaxy = lines.iter()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    galaxy
}


fn calc_total_dist(lines : Vec<&str>, empty_dist:usize) -> usize {
    let galaxy_map = parse(lines);
    let galaxies:Vec<(usize,usize)> = galaxy_map.iter().enumerate()
        .map(|(y,row)| row.iter().enumerate()
            .filter(|(_,c)| **c == '#')
            .map(|(x, _)| (x ,y)).collect::<Vec<(usize,usize)>>())
        .flatten()
        .collect();

    let mut empty_cols = vec![];
    let mut empty_rows = vec![];

    // Check empty columns
    for x in 0..galaxy_map.first().unwrap().len() {
        let is_empty = (0..galaxy_map.len()).into_iter().map(|y| galaxy_map.get(y).unwrap().get(x).unwrap()).all(|c| *c == '.');
        if is_empty {
            empty_cols.push(x);
        }
    }

    // Check empty rows
    for y in 0..galaxy_map.len() {
        let is_empty = (0..galaxy_map.first().unwrap().len()).into_iter().map(|x| galaxy_map.get(y).unwrap().get(x).unwrap()).all(|c| *c == '.');
        if is_empty {
            empty_rows.push(y);
        }
    }

    let mut sum = 0;
    for galaxy_no in 0..galaxies.len() {
        let (x0,y0) = galaxies[galaxy_no];
        for other_no in galaxy_no..galaxies.len() {
            if galaxy_no != other_no {
                let (x1,y1) = galaxies[other_no];
                // Calc distance and count number of empty passes
                let x_start = min(x0, x1);
                let x_end = max(x0, x1);
                let y_start = min(y0, y1);
                let y_end = max(y0, y1);

                let number_passes_col = empty_cols.iter().filter(|x| **x >= x_start && **x < x_end).count();
                let x_diff = x_end - x_start;
                sum += number_passes_col * empty_dist + (x_diff - number_passes_col);

                let number_passes_row = empty_rows.iter().filter(|y| **y >= y_start && **y < y_end).count();
                let y_diff = y_end - y_start;
                sum += number_passes_row * empty_dist + (y_diff - number_passes_row);
            }
        }
    }

    sum
}
fn part1(lines : Vec<&str>) -> String {
    calc_total_dist(lines, 2).to_string()
}

fn part2(lines : Vec<&str>) -> String {
    calc_total_dist(lines, 1000000).to_string()
}


#[cfg(test)]
mod tests {
    use std::assert_eq;
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use util::Part::{Part1, Part2};


    const INPUT:&str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

    #[test]
    fn test1() {
        assert_eq!("374", solve(INPUT.to_string(), Part1));
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../../input/input_11.txt");
        assert_eq!("9591768", solve(input.to_string(), Part1));
    }

    #[test]
    fn test2() {
        assert_eq!("82000210", solve(INPUT.to_string(), Part2));
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../../input/input_11.txt");
        assert_eq!("746962097860", solve(input.to_string(), Part2));
    }
}
