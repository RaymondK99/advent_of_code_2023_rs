use std::collections::VecDeque;
use super::Part;

pub fn solve(input : String, part: Part) -> String {
    let lines = input.lines().collect();
    match part {
        Part::Part1 => part1(lines),
        Part::Part2 => part2(lines)
    }
}

#[derive(Debug)]
struct Card{
    _card_no:u32,
    winning:Vec<u32>,
    numbers:Vec<u32>,
}

impl Card {
    fn new(input:&str) -> Card {
        let mut it = input.split(|c| c == '|');
        let mut first:VecDeque<u32> = it.next().unwrap().split(|c| c == ' ' || c== ':')
            .into_iter()
            .filter(|s| s.starts_with(|c:char| c.is_digit(10)))
            .map(|s| s.parse::<u32>().unwrap())
            .collect();

        let second = it.next().unwrap().split(|c| c == ' ' || c== ':')
            .into_iter()
            .filter(|s| s.starts_with(|c:char| c.is_digit(10)))
            .map(|s| s.parse::<u32>().unwrap())
            .collect();

        Card{ _card_no:first.pop_front().unwrap(), winning:first.iter().copied().collect(), numbers:second}
    }

    fn matches(&self) -> usize {
        self.numbers.iter().filter(|num| self.winning.contains(num)).count()
    }
    fn points(&self) -> usize {
        let matches = self.matches();
        if matches == 0 {
            return 0;
        } else {
            let base:usize = 2;
            return base.pow((matches - 1) as u32);
        }
    }
}

fn part1(lines : Vec<&str>) -> String {
    lines.iter()
        .map(|s| Card::new(s))
        .map(|card| card.points()).sum::<usize>()
        .to_string()
}

fn part2(lines : Vec<&str>) -> String {
    let cards:Vec<Card> =  lines.iter()
        .map(|s| Card::new(s))
        .collect();

    let mut number_of_cards = vec![1; cards.len()];

    cards.iter().enumerate().for_each(|(index, card)| {
        for i in 0..card.matches() {
            number_of_cards[index + i + 1] += number_of_cards[index];
        }
    });

    number_of_cards.iter().sum::<u32>().to_string()
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use util::Part::{Part1, Part2};

    const INPUT:&str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn test1() {
        assert_eq!("13", solve(INPUT.to_string(), Part1));
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../../input/input_04.txt");
        assert_eq!("28750", solve(input.to_string(), Part1));
    }

    #[test]
    fn test2() {
        assert_eq!("30", solve(INPUT.to_string(), Part2));
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../../input/input_04.txt");
        assert_eq!("10212704", solve(input.to_string(), Part2));
    }
}
