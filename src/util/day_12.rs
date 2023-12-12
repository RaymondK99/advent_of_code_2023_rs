use std::collections::HashMap;
use super::Part;

pub fn solve(input : String, part: Part) -> String {
    let lines = input.lines().collect();
    match part {
        Part::Part1 => part1(lines),
        Part::Part2 => part2(lines)
    }
}


fn parse_line(line:&str, folds:usize) -> (Vec<char>, Vec<usize>) {
    let mut it = line.split(' ');
    let pattern = it.next().unwrap().chars().collect::<Vec<char>>();
    let damaged = it.next().unwrap().split(',').map(|s| s.parse::<usize>().unwrap()).collect::<Vec<usize>>();
    let mut unfolded_pattern = vec![];
    let mut unfolded_damaged = vec![];

    for i in 0..folds {
        for j in 0..pattern.len() {
            unfolded_pattern.push(pattern[j]);
        }
        if i < folds - 1 {
            unfolded_pattern.push('?');
        }

        for j in 0..damaged.len() {
            unfolded_damaged.push(damaged[j]);
        }
    }

    (unfolded_pattern, unfolded_damaged)
}

fn permutations_for_pattern(pattern:&[char], damaged:&[usize], acc_sequence_len:usize, cache:&mut HashMap<(String, Vec<usize>, usize), usize>) -> usize {
    let pattern_str:String = pattern.iter().collect::<String>();
    let dam_str = damaged.iter().copied().collect::<Vec<usize>>();
    let key = (pattern_str, dam_str, acc_sequence_len);

    if cache.contains_key(&key) {
        return *cache.get(&key).unwrap();
    }

    let result = if pattern.is_empty() && damaged.is_empty() && acc_sequence_len == 0 {
        1
    } else {
        if acc_sequence_len == 0 {
            if pattern.is_empty() && !damaged.is_empty() {
                0
            } else {
                if pattern[0] == '.' {
                    // Operational, ok
                    permutations_for_pattern(&pattern[1..], damaged, acc_sequence_len, cache)
                } else if pattern[0] == '#' {
                    // Damaged
                    if damaged.is_empty() {
                        // Does not match
                        0
                    } else {
                        // Match a sequence of damaged springs
                        permutations_for_pattern(&pattern[1..], damaged, acc_sequence_len + 1, cache)
                    }
                } else {
                    // Unknown, we can either start a sequence or not
                    if damaged.is_empty() {
                        // Damaged slice is empty, it cant be damaged
                        permutations_for_pattern(&pattern[1..], damaged, acc_sequence_len, cache)
                    } else {
                        // Assume its damaged or operational
                        // be greedy and try to consume damage list first..
                        let branch_one = permutations_for_pattern(&pattern[1..], damaged, acc_sequence_len + 1, cache);
                        let branch_two = permutations_for_pattern(&pattern[1..], damaged, acc_sequence_len, cache);
                        branch_one + branch_two
                    }
                }
            }
        } else {
            // We are inside a sequence
            // Can we close the sequence
            if damaged[0] == acc_sequence_len {

                // We should try and close the sequence
                if pattern.is_empty() {
                    // Ok last pattern
                    permutations_for_pattern(pattern, &damaged[1..], 0, cache)
                } else {
                    if pattern[0] == '.' || pattern[0] == '?' {
                        // This is ok, sequence is closed
                        permutations_for_pattern(&pattern[1..], &damaged[1..], 0, cache)
                    } else {
                        // Not ok...
                        0
                    }
                }
            } else {
                // Continue sequence
                if pattern.is_empty() {
                    // No match
                    0
                } else {
                    if pattern[0] == '#' || pattern[0] == '?' {
                        permutations_for_pattern(&pattern[1..], damaged, acc_sequence_len + 1, cache)
                    } else {
                        // We cant close sequence
                        0
                    }
                }
            }
        }
    };


    cache.insert(key, result);
    result
}


fn calc_total_permutations(lines : Vec<&str>, folds:usize) -> usize {
    lines.iter()
        .map(|line| parse_line(line, folds))
        .map(|(patterns, damaged)|  permutations_for_pattern(patterns.as_slice(), damaged.as_slice(), 0, &mut HashMap::new()))
        .sum::<usize>()
}

fn part1(lines : Vec<&str>) -> String {
    calc_total_permutations(lines, 1).to_string()
}


fn part2(lines : Vec<&str>) -> String {
    calc_total_permutations(lines, 5).to_string()
}


#[cfg(test)]
mod tests {
    use std::assert_eq;
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use util::Part::{Part1, Part2};


    const INPUT:&str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";


    #[test]
    fn test1() {
        assert_eq!("21", solve(INPUT.to_string(), Part1));
    }


    #[test]
    fn test_part1() {
        let input = include_str!("../../input/input_12.txt");
        assert_eq!("8193", solve(input.to_string(), Part1));
    }

    #[test]
    fn test2() {
        assert_eq!("525152", solve(INPUT.to_string(), Part2));
    }


    #[test]
    fn test_part2() {
        let input = include_str!("../../input/input_12.txt");
        assert_eq!("45322533163795", solve(input.to_string(), Part2));
    }
}
