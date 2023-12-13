use std::usize;
use super::Part;

pub fn solve(input : String, part: Part) -> String {
    let lines = input.split("\n\n").collect();
    match part {
        Part::Part1 => part1(lines),
        Part::Part2 => part2(lines)
    }
}


fn parse_pattern(pattern:&str) -> Vec<Vec<char>> {
    pattern.lines().map(|line| line.chars().collect::<Vec<char>>()).collect()
}

fn transpose(pattern:&Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut transposed = vec![];
    for x in 0..pattern.first().unwrap().len() {
        let mut row = vec![];
        for y in 0..pattern.len() {
            row.push(*pattern.get(y).unwrap().get(x).unwrap());
        }
        transposed.push(row);
    }
    transposed
}

fn find_pattern(pattern:&Vec<Vec<char>>) -> Vec<usize> {
    let mut result = vec![];
    for y in 1..pattern.len() {
        let mut y1 = y;
        let mut y2 = y;
        let mut reflection = true;
        while reflection && y1 > 0 && y2 < pattern.len() {
            reflection = reflection && pattern.get(y1-1).unwrap().eq(pattern.get(y2).unwrap());
            y1 -= 1;
            y2 += 1;
        }

        if reflection {
            result.push(y);
        }
    }

    return result;
}

fn get_score(matrix: &Vec<Vec<char>>) -> Vec<usize> {
    let mut result = vec![];
    result.append(&mut find_pattern(&matrix).into_iter().filter(|row| *row > 0).map(|row| row * 100).collect());
    result.append(&mut find_pattern(&transpose(&matrix)).into_iter().filter(|col| *col > 0).collect());
    result
}

fn part1(input : Vec<&str>) -> String {
    input.into_iter()
        .map(|pattern| parse_pattern(pattern))
        .map(|matrix|  get_score(&matrix)[0])
        .sum::<usize>()
        .to_string()
}


fn part2(input : Vec<&str>) -> String {
    let mut sum = 0;
    for pattern in input.iter() {
        let mut matrix = parse_pattern(pattern);
        let old_score = get_score(&matrix)[0];
        'outer_loop: for y in 0..matrix.len() {
            for x in 0..matrix.first().unwrap().len() {
                let old_char = matrix[y][x];
                let next_char = match old_char {
                    '.' => '#',
                    _ => '.',
                };

                matrix[y][x] = next_char;

                // Check for score after altering matrix
                let scores = get_score(&matrix).iter()
                    .filter(|s| **s != old_score)
                    .copied()
                    .collect::<Vec<usize>>();

                // Restore matrix
                matrix[y][x] = old_char;

                if scores.len() == 1 {
                    sum += scores[0];
                    break 'outer_loop;
                }
            }
        }
    }

    sum.to_string()

}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use util::Part::{Part1, Part2};


    const INPUT:&str = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

    #[test]
    fn test1() {
        assert_eq!("405", solve(INPUT.to_string(), Part1));
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../../input/input_13.txt");
        assert_eq!("28651", solve(input.to_string(), Part1));
    }


    #[test]
    fn test2() {
        assert_eq!("400", solve(INPUT.to_string(), Part2));
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../../input/input_13.txt");
        assert_eq!("25450", solve(input.to_string(), Part2));
    }
}
