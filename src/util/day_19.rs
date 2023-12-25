use std::collections::VecDeque;
use std::fmt::Debug;
use util::day_19::RuleResult::{Accepted, Rejected};
use super::Part;

pub fn solve(input : String, part: Part) -> String {
    let mut items:Vec<Vec<&str>> = input.split("\n\n")
        .map(|lines| lines.lines().collect()).collect();
    match part {
        Part::Part1 => part1(items.remove(0), items.remove(0)),
        Part::Part2 => part2(items.remove(0), items.remove(0))
    }
}

#[derive(Debug)]
struct MachinePart {
    x:u32,
    m:u32,
    a:u32,
    s:u32,
}

impl MachinePart {
    fn get_field_value(&self, ch:char) -> u32 {
        match ch {
            'x' => self.x,
            'm' => self.m,
            'a' => self.a,
            's' => self.s,
            _ => panic!(".."),
        }
    }

    fn sum(&self) -> u32 {
        self.a + self.m + self.x + self.s
    }
}



#[derive(Debug)]
enum Condition {
    GreaterThan(char, u32),
    LessThan(char, u32),
}

impl Condition {

    fn new(s:&str) -> Condition {
        let mut it = s.chars();
        let ch = it.next().unwrap();
        let cond = it.next().unwrap();
        let num = &s[2..].parse::<u32>().unwrap();
        if cond.eq(&'<') {
            Condition::LessThan(ch, *num)
        } else {
            Condition::GreaterThan(ch, *num)
        }
    }

    fn check(&self, machine_part:&MachinePart) -> bool {
        let field_value = machine_part.get_field_value(self.get_field());

        match *self {
            Condition::GreaterThan(_, rule_value) => rule_value < field_value,
            Condition::LessThan(_, rule_value) => rule_value > field_value,
        }
    }

    fn get_field(&self) -> char {
        match *self {
            Condition::GreaterThan(ch, _) => ch,
            Condition::LessThan(ch, _) => ch,
        }
    }


    fn apply_interval(&self, intervals:&Vec<Vec<u32>>) -> (Vec<Vec<u32>>, Vec<Vec<u32>>) {
        let mut matched_intervals = vec![];
        let mut unmatched_intervals = vec![];
        for i in 0..intervals.len() {
            let ch = match i {
                0 => 'x',
                1 => 'm',
                2 => 'a',
                3 => 's',
                _ => panic!("..."),
            };

            if ch == self.get_field() {
                let (matched, unmatched) = self.apply(&intervals[i]);
                matched_intervals.push(matched);
                unmatched_intervals.push(unmatched);
            } else {
                matched_intervals.push(intervals[i].iter().copied().collect());
                unmatched_intervals.push(intervals[i].iter().copied().collect())
            }
        }

        (matched_intervals, unmatched_intervals)
    }
    fn apply(&self, interval:&Vec<u32>) -> (Vec<u32>, Vec<u32>) {
        match *self {
            Condition::GreaterThan(_, value) => {
                let matched = interval.iter().filter(| n| **n > value).copied().collect();
                let unmatched = interval.iter().filter(| n| **n <= value).copied().collect();
                (matched, unmatched)
            }
            Condition::LessThan(_, value) => {
                let matched = interval.iter().filter(| n| **n < value).copied().collect();
                let unmatched = interval.iter().filter(| n| **n >= value).copied().collect();
                (matched, unmatched)
            }
        }
    }
}

#[derive(Debug)]
struct Rule {
    name:String,
    conditions:Vec<(Condition, RuleResult)>,
    default_rule:RuleResult,
}

impl Rule {
    fn new(line:&str) -> Rule {
        let mut fields = line.split(|c| c == '{' || c == '}' || c == ',').filter(|s| !s.is_empty()).collect::<VecDeque<&str>>();
        let name = fields.pop_front().unwrap().to_string();
        let default_rule = RuleResult::new(fields.pop_back().unwrap());
        let mut conditions = vec![];
        for field in fields {
            for rule_str in field.split(',') {
                let mut it = rule_str.split(':');
                conditions.push((Condition::new(it.next().unwrap()), RuleResult::new(it.next().unwrap())));
            }
        }
        Rule{name, conditions, default_rule}
    }

    fn check(&self, machine_part:&MachinePart) -> RuleResult {
        for (condition, res) in self.conditions.iter() {
            if condition.check(machine_part) {
                return res.clone()
            }
        }

        self.default_rule.clone()
    }


    fn apply_interval(&self, interval:Vec<Vec<u32>>) -> Vec<(Vec<Vec<u32>>, RuleResult)> {
        let mut output = vec![];
        let mut stack = VecDeque::new();
        stack.push_back(interval);
        for (cond, rule_result) in self.conditions.iter() {
            let next = stack.pop_front().unwrap();
            let (matched, unmatched) = cond.apply_interval(&next);

            output.push((matched, rule_result.clone()));
            stack.push_front(unmatched);
        }

        output.push((stack.pop_front().unwrap(), self.default_rule.clone()));
        output
    }
}

#[derive(Debug, Clone)]
enum RuleResult {
    Accepted,
    Rejected,
    Rule(String),
}

impl RuleResult {
    fn new(s:&str) -> RuleResult {
        if s.eq("A") {
            Accepted
        } else if s.eq("R") {
            Rejected
        } else {
            RuleResult::Rule(s.to_string())
        }
    }
}


impl MachinePart {
    fn new(line:&str) -> MachinePart {
        let numbers = line.split(|ch|  ch == '{' || ch == '}' || ch == '=' || ch == ',')
            .filter(|s| !s.is_empty())
            .filter(|s| s.starts_with(|c:char| c.is_digit(10)))
            .map(|s| s.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();
        MachinePart{x:numbers[0], m:numbers[1], a:numbers[2], s:numbers[3]}
    }
}



fn match_rule(part:&MachinePart, rules:&Vec<Rule>) -> bool {
    let mut next = rules.iter().find(|rule| rule.name.as_str().eq("in")).unwrap();
    loop {
        let rule = next.check(part);
        match rule {
            Accepted => {
                return true;
            },
            Rejected => {
                return false;
            }
            RuleResult::Rule(rule_name) => {
                next = &rules.iter().find(|r| r.name.eq(&rule_name)).unwrap();
            },
        }
    }
}

fn match_interval(rules:&Vec<Rule>) -> usize {
    let v:Vec<u32> = (1..=4000).into_iter().collect();
    let mut accepted = 0;
    let mut queue = VecDeque::new();
    queue.push_back((vec![v.clone(), v.clone(), v.clone(), v.clone()], "in".to_string()));

    while !queue.is_empty() {
        let (intervals, rule_name) = queue.pop_front().unwrap();
        let rule = rules.iter().find(|rule| rule.name.as_str().eq(rule_name.as_str())).unwrap();
        let results = rule.apply_interval(intervals);

        for (interval, res) in results {
            match res {
                Accepted => {
                    accepted += interval.iter().map(|v| v.len()).product::<usize>();
                },
                RuleResult::Rule(next_rule_name) => {
                    queue.push_back((interval, next_rule_name));
                },
                _ => {},
            }
        }
    }

    accepted
}
fn part1(input_rules : Vec<&str>, input_parts : Vec<&str>) -> String {
    let parts:Vec<MachinePart> = input_parts.iter().map(|line| MachinePart::new(line)).collect();
    let rules:Vec<Rule> = input_rules.into_iter().map(|s| Rule::new(s)).collect();

    parts.iter()
        .filter(|part| match_rule(part, &rules))
        .map(|part| part.sum())
        .sum::<u32>()
        .to_string()
}

fn part2(input_rules : Vec<&str>, _input_parts : Vec<&str>) -> String {
    let rules:Vec<Rule> = input_rules.into_iter().map(|s| Rule::new(s)).collect();
    match_interval(&rules).to_string()
}


#[cfg(test)]
mod tests {
    use std::assert_eq;
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use util::Part::{Part1, Part2};


    const INPUT:&str = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";

    #[test]
    fn test1() {
        assert_eq!("19114", solve(INPUT.to_string(), Part1));
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../../input/input_19.txt");
        assert_eq!("319295", solve(input.to_string(), Part1));
    }

    #[test]
    fn test2() {
        assert_eq!("167409079868000", solve(INPUT.to_string(), Part2));
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../../input/input_19.txt");
        assert_eq!("110807725108076", solve(input.to_string(), Part2));
    }
}
