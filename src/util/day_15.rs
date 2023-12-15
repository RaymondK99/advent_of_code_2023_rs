use super::Part;

pub fn solve(input : String, part: Part) -> String {
    let lines = input.split(',').into_iter().collect();
    match part {
        Part::Part1 => part1(lines),
        Part::Part2 => part2(lines)
    }
}

fn hash_chars(str:&str) -> usize {
    str.chars()
        .fold(0, |prev, ch|  ((prev + ch as usize) * 17 ) % 256)
}

fn part1(ops: Vec<&str>) -> String {
    ops.iter()
        .map(|line| hash_chars(line))
        .sum::<usize>()
        .to_string()
}

fn part2(ops : Vec<&str>) -> String {
    let mut boxes:Vec<Vec<(&str, usize)>> = (0..256).into_iter().map(|_| Vec::new()).collect();

    // Run operations
    for operation in ops {
        let mut it = operation.split(|c| c == '=' || c == '-');
        let label = it.next().unwrap();
        let curr_box = boxes.get_mut(hash_chars(label)).unwrap();
        let index_opt =  curr_box.iter().enumerate()
            .find(|(_, (l, _))| l.eq(&label))
            .map(|(index, _)| index);

        if operation.contains("-") {
            // Remove
            match index_opt {
                None => {},
                Some(index) => { curr_box.remove(index);}
            }
        } else {
            // Add or update
            let focal_len = operation.split('=').last().unwrap().parse::<usize>().unwrap();

            // Remove
            match index_opt {
                None => {
                    curr_box.push((label, focal_len));
                },
                Some(index) => {
                    curr_box[index].1 = focal_len;
                }
            }
        }
    }

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
