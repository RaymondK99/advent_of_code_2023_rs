use super::Part;

pub fn solve(input : String, part: Part) -> String {
    let lines = input.lines().collect();
    match part {
        Part::Part1 => part1(lines),
        Part::Part2 => part2(lines)
    }
}

fn calc_next_value(mut numbers:Vec<i32>, part2:bool) -> Vec<i32> {
    let mut history:Vec<Vec<i32>> = vec![];

    // Add first line of readings, reverse if its part 2
    if part2 {
        numbers.reverse();
    }
    history.push(numbers);


    // While not all zeroes, add next later
    while !history.last().unwrap().iter().all(|n | *n == 0) {
        let last = history.last().unwrap();
        let mut next = vec![];
        for i in 1..last.len() {
            next.push( last[i] - last[i-1]);
        }
        history.push(next);
    }

    for i in 0..history.len() {
        let index = history.len() - 1 - i;

        if index == history.len() - 1 {
            // add a zero in the last row
            history.get_mut(index).unwrap().push(0);
        } else {
            // Calculate next value for row
            let last_in_row = *history[index].last().unwrap();
            let last_diff = *history[index+1].last().unwrap();
            history.get_mut(index).unwrap().push(last_diff + last_in_row);
        }
    }

    history.remove(0)
}

fn calc_sum_of_extrapolated_values(lines : Vec<&str>, part2:bool) -> i32 {
    lines.iter()
        .map(|s| s.split(' ')
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<i32>>())
        .map(|row| calc_next_value(row, part2))
        .map(|row| *row.last().unwrap())
        .sum::<i32>()
}

fn part1(lines : Vec<&str>) -> String {
    calc_sum_of_extrapolated_values(lines, false).to_string()
}

fn part2(lines : Vec<&str>) -> String {
    calc_sum_of_extrapolated_values(lines, true).to_string()
}


#[cfg(test)]
mod tests {
    use std::assert_eq;
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use util::Part::{Part1, Part2};


    const INPUT:&str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    #[test]
    fn test1() {
        assert_eq!("114", solve(INPUT.to_string(), Part1));
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../../input/input_09.txt");
        assert_eq!("2008960228", solve(input.to_string(), Part1));
    }

    #[test]
    fn test2() {
        assert_eq!("2", solve(INPUT.to_string(), Part2));
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../../input/input_09.txt");
        assert_eq!("1097", solve(input.to_string(), Part2));
    }
}
