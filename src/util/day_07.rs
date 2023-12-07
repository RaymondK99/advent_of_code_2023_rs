use std::char;
use util::day_07::HandType::{FIVE, FOUR, FullHouse, PAIR, THREE, HighCard, TwoPairs};
use super::Part;

pub fn solve(input : String, part: Part) -> String {
    let lines = input.lines().collect();
    match part {
        Part::Part1 => part1(lines),
        Part::Part2 => part2(lines)
    }
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
struct Hand {
    hand_type:u32,
    hand_value:usize,
    cards:Vec<char>,
    bid:u32,
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
enum HandType {
    FIVE = 7,
    FOUR = 6,
    FullHouse = 5,
    THREE = 4,
    TwoPairs = 3,
    PAIR = 2,
    HighCard = 1,
}

impl Hand {

    const CARDS_VALUES:[char;13] = ['A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2'];
    const CARDS_VALUES_PART2:[char;13] = ['A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J'];

    fn new(line:&str, part2:bool) -> Hand {
        let mut it = line.split(' ');
        let cards = it.next().unwrap().chars().collect();
        let bid = it.next().unwrap().parse().unwrap();
        let hand_type = Hand::get_hand_type(&cards, part2) as u32;

        let mut hand_value = 0;
        for i in 0..5 {
            hand_value += Hand::get_card_value(cards[i], part2);
            if i < 4 {
                hand_value *= Hand::CARDS_VALUES.len();
            }
        }

        Hand{hand_type, hand_value, cards, bid}
    }

    fn get_card_value(card:char, part2:bool) -> usize {
        let card_values = if part2 {
            Hand::CARDS_VALUES_PART2
        } else {
            Hand::CARDS_VALUES
        };

        card_values.iter().rev()
            .enumerate()
            .find(|(_, char)| card == **char)
            .map(|(index, _)| index)
            .unwrap()
    }

    fn get_hand_type_no_jokers(cards:&Vec<char>) -> HandType {
        let mut array:[u8;u8::MAX as usize] = [0;u8::MAX as usize];
        for i in 0..cards.len() {
            array[cards[i] as usize] += 1;
        }

        let fives = array.iter().filter(|v| **v == 5).count();
        let fours = array.iter().filter(|v| **v == 4).count();
        let threes = array.iter().filter(|v| **v == 3).count();
        let pairs = array.iter().filter(|v| **v == 2).count();

        if fives == 1 {
            FIVE
        } else if fours == 1 {
            FOUR
        } else if threes == 1 && pairs == 1 {
            FullHouse
        } else if threes == 1 {
            THREE
        } else if pairs == 2 {
            TwoPairs
        } else if pairs == 1 {
            PAIR
        } else {
            HighCard
        }
    }

    fn get_hand_type(cards:&Vec<char>, part2: bool) -> HandType {

        let cards_no_jokers:Vec<char> = cards.iter().filter(|c| **c != 'J').copied().collect();
        let num_jokers = if part2 {
            5 - cards_no_jokers.len()
        } else {
            0
        };

        return if num_jokers == 0 {
            Hand::get_hand_type_no_jokers(cards)
        } else {
            // Get hand type with no jokers
            let hand_type_base = Hand::get_hand_type_no_jokers(&cards_no_jokers);

            if num_jokers == 1 {
                match hand_type_base {
                    FullHouse => FOUR,
                    THREE => FOUR,
                    TwoPairs => FullHouse,
                    PAIR => THREE,
                    HighCard => PAIR,
                    _ => FIVE,
                }
            } else if num_jokers == 2 {
                match hand_type_base {
                    TwoPairs => FOUR,
                    PAIR => FOUR,
                    HighCard => THREE,
                    _ => FIVE,
                }
            } else if num_jokers == 3 {
                match hand_type_base {
                    HighCard => FOUR,
                    _ => FIVE,
                }
            } else {
                FIVE
            }
        }
    }
}

fn play_game(mut hands:Vec<Hand>) -> usize {
    hands.sort();
    hands.iter().enumerate()
        .map(|(rank, hand)| (rank + 1) * hand.bid as usize)
        .sum::<usize>()
}
fn part1(lines : Vec<&str>) -> String {
    play_game(lines.iter().map(|line| Hand::new(line, false)).collect()).to_string()
}

fn part2(lines : Vec<&str>) -> String {
    play_game(lines.iter().map(|line| Hand::new(line, true)).collect()).to_string()
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use util::Part::{Part1, Part2};


    const INPUT:&str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    fn test1() {
        assert_eq!("6440", solve(INPUT.to_string(), Part1));
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../../input/input_07.txt");
        assert_eq!("248569531", solve(input.to_string(), Part1));
    }

    #[test]
    fn test2() {
        assert_eq!("5905", solve(INPUT.to_string(), Part2));
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../../input/input_07.txt");
        assert_eq!("250382098", solve(input.to_string(), Part2));
    }
}
