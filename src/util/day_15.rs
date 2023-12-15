use util::day_15::Operation::{ADD, REMOVE};
use super::Part;

pub fn solve(input : String, part: Part) -> String {
    let lines = input.split(',').into_iter().collect();
    match part {
        Part::Part1 => part1(lines),
        Part::Part2 => part2(lines)
    }
}

#[derive(Debug)]
enum Operation {
    ADD(String, String, usize),
    REMOVE(String, String),
}

impl Operation {
    fn new(op_str:&str) -> Operation {
        let mut it = op_str.split(|c| c == '=' || c == '-');
        let label = it.next().unwrap().to_string();
        if op_str.contains("-") {
            REMOVE(op_str.to_string(), label)
        } else {
            ADD(op_str.to_string(), label, it.next().unwrap().parse::<usize>().unwrap())
        }
    }

    fn get_label(&self) -> &String {
        match self {
            ADD(_, label, _) => label,
            REMOVE(_, label) => label,
        }
    }

    fn hash_chars(str:&str) -> usize {
        str.chars()
            .fold(0, |prev, ch|  ((prev + ch as usize) * 17 ) % 256)
    }

    fn hash_operation(&self) -> usize {
        match self {
            REMOVE(s, _) => Operation::hash_chars(s.as_str()),
            ADD(s, _,_) => Operation::hash_chars(s.as_str()),
        }
    }

    fn hash_label(&self) -> usize {
        match self {
            REMOVE(_, s) => Operation::hash_chars(s.as_str()),
            ADD(_, s,_) => Operation::hash_chars(s.as_str()),
        }
    }
}

fn part1(ops: Vec<&str>) -> String {
    ops.iter()
        .map(|line| Operation::new(line).hash_operation())
        .sum::<usize>()
        .to_string()
}

fn part2(ops : Vec<&str>) -> String {
    let mut boxes:Vec<Vec<(String, usize)>> = (0..256).into_iter().map(|_| Vec::new()).collect();
    ops.iter().map(|line| Operation::new(line))
        .for_each(|op| {
            let curr_box = boxes.get_mut(op.hash_label()).unwrap();
            let index_opt =  curr_box.iter().enumerate()
                .find(|(_, (l, _))| l.eq(op.get_label()))
                .map(|(index, _)| index);

            match op {
                ADD(_, label, focal_len) => {
                    if index_opt.is_some() {
                        curr_box[index_opt.unwrap()].1 = focal_len;
                    } else {
                        curr_box.push((label.clone(), focal_len));
                    }
                }
                REMOVE(_, _) => {
                    if index_opt.is_some() {
                        curr_box.remove(index_opt.unwrap());
                    }
                }
            }
        });

    // Sum focal power
    boxes.into_iter().enumerate()
        .map(|(box_no, curr_box)| curr_box.into_iter().enumerate()
            .map(|(box_pos, (_, focal_len))| (box_no + 1) * (box_pos + 1) * focal_len)
            .sum::<usize>())
        .sum::<usize>().to_string()
}


#[cfg(test)]
mod tests {
    use std::assert_eq;
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use util::Part::{Part1, Part2};

    const INPUT:&str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn test1() {
        assert_eq!("1320", solve(INPUT.to_string(), Part1));
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../../input/input_15.txt");
        assert_eq!("516657", solve(input.to_string(), Part1));
    }

    #[test]
    fn test2() {
        assert_eq!("145", solve(INPUT.to_string(), Part2));
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../../input/input_15.txt");
        assert_eq!("210906", solve(input.to_string(), Part2));
    }
}
