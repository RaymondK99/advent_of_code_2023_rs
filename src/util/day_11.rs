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
    let mut galaxy = lines.iter()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut empty_cols = vec![];
    let mut empty_rows = vec![];

    // Check empty columns
    for x in 0..galaxy.first().unwrap().len() {
        let is_empty = (0..galaxy.len()).into_iter().map(|y| galaxy.get(y).unwrap().get(x).unwrap()).all(|c| *c == '.');
        if is_empty {
           empty_cols.push(x);
        }
    }

    // Check empty rows
    for y in 0..galaxy.len() {
        let is_empty = (0..galaxy.first().unwrap().len()).into_iter().map(|x| galaxy.get(y).unwrap().get(x).unwrap()).all(|c| *c == '.');
        if is_empty {
            empty_rows.push(y);
        }
    }

    // Insert 'x' as extra columns
    for y in 0..galaxy.len() {
        for (i, col_no) in empty_cols.iter().enumerate() {
            galaxy.get_mut(y).unwrap().insert(col_no + i, 'x');
        }
    }

    // Insert 'y' as extra columns
    for (i, row_no) in empty_rows.into_iter().enumerate() {
        let new_row = (0..galaxy.first().unwrap().len()).map(|_| 'y').collect::<Vec<char>>();
        galaxy.insert(row_no + i, new_row);

    }

    galaxy
}


fn calc_dist(x0:usize, y0:usize, x1:usize, y1:usize, dist:usize, galaxy:&Vec<Vec<char>>) -> usize {
    let x_start = min(x0, x1);
    let x_end = max(x0, x1);
    let y_start = min(y0, y1);
    let y_end = max(y0, y1);

    let x_diff = (x_start..x_end)
        .map(|x| galaxy.get(y_start).unwrap().get(x).unwrap())
        .map(|ch| {
            match *ch {
                'x' => dist,
                _ => 1,
            }
        }).sum::<usize>();

    let y_diff = (y_start..y_end)
        .map(|y| galaxy.get(y).unwrap().get(x_start).unwrap())
        .map(|ch| {
            match *ch {
                'y' => dist,
                _ => 1,
            }
        }).sum::<usize>();

    x_diff + y_diff
}

fn calc_total_dist(lines : Vec<&str>, empty_dist:usize) -> usize {
    let galaxy_map = parse(lines);
    let galaxies:Vec<(usize,usize)> = galaxy_map.iter().enumerate()
        .map(|(y,row)| row.iter().enumerate()
            .filter(|(_,c)| **c == '#')
            .map(|(x, _)| (x ,y)).collect::<Vec<(usize,usize)>>())
        .flatten()
        .collect();

    let mut sum = 0;
    for galaxy_no in 0..galaxies.len() {
        let (x0,y0) = galaxies[galaxy_no];
        for other_no in galaxy_no..galaxies.len() {
            if galaxy_no != other_no {
                let (x1,y1) = galaxies[other_no];
                // Calc manhattan dist
                sum += calc_dist(x0, y0, x1, y1, empty_dist - 1, &galaxy_map);
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
