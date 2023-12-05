use std::cmp::{max, min};
use std::collections::VecDeque;
use super::Part;

pub fn solve(input : String, part: Part) -> String {
    let parts = input.split("\n\n").collect();
    match part {
        Part::Part1 => part1(parts),
        Part::Part2 => part2(parts)
    }
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
struct Interval {
    start:i64,
    end:i64,
}

impl Interval {
    fn new(start:i64, end:i64, delta:i64) -> Interval {
        Interval{start:start + delta, end:end + delta}
    }
}


#[derive(Debug)]
struct Mapper {
    destination:i64,
    source:i64,
    size:i64,
}



impl Mapper {
    fn new(line:&str) -> Mapper {
        let mut it = line.split(' ').into_iter().map(|s| s.parse::<i64>().unwrap());
        let destination = it.next().unwrap();
        let source = it.next().unwrap();
        let size = it.next().unwrap();
        Mapper{destination, source, size}
    }

    fn intersects_single(&self, source:i64) -> bool {
        source >= self.source && source < self.source + self.size
    }

    fn transform_single(&self, source:i64) -> i64 {
        self.destination - self.source + source
    }

    fn intersects_interval(&self, interval:&Interval) -> bool {
        let source_start = self.source;
        let source_end = self.source + self.size - 1;
        return  !(interval.end < source_start || interval.start > source_end)
    }

    fn transform_interval(&self, interval:&Interval) -> (Vec<Interval>, Vec<Interval>) {
        let source_start = self.source;
        let source_end = self.source + self.size - 1;
        let delta = self.destination - self.source;

        let mut mapped_interval = vec![];
        let mut unmapped_interval = vec![];

        // is interval before or after mapper?
        if interval.start < source_start {
            // Before part
            unmapped_interval.push(Interval::new(interval.start, source_start-1, 0 ));
        }

        // Overlapping part
        mapped_interval.push(Interval::new(max(source_start, interval.start), min(source_end, interval.end), delta));

        // does interval go beyond mapper?
        if interval.end > source_end {
            // After part
            unmapped_interval.push(Interval::new(source_end+1, interval.end, 0));
        }

        (unmapped_interval, mapped_interval)
    }
}


#[derive(Debug)]
struct Layer {
    mappers: Vec<Mapper>
}

impl Layer {
    fn new(mut lines:VecDeque<&str>) -> Layer {
        // pop first
        lines.pop_front();
        Layer { mappers:lines.into_iter().map(|line| Mapper::new(line)).collect()}
    }

    fn convert(&self, source:i64) -> i64 {
        for interval in self.mappers.iter() {
            if interval.intersects_single(source) {
                return interval.transform_single(source);
            }
        }
        source
    }

    fn intersects(&self, interval:&Interval) -> bool {
        self.mappers.iter().filter(|mapper| mapper.intersects_interval(interval)).count() > 0
    }
    fn transform(&self, interval:Interval) -> Vec<Interval> {
        let mut transformed_intervals = vec![];
        let mut unmapped_intervals = VecDeque::new();

        // Add original interval to be evaluated
        unmapped_intervals.push_back(interval);

        while !unmapped_intervals.is_empty() {
            let unmapped_interval =  unmapped_intervals.pop_front().unwrap();

            if self.intersects(&unmapped_interval) {
                // Transform it
                for mapper in self.mappers.iter() {
                    if mapper.intersects_interval(&unmapped_interval) {
                        let (unmapped, mut mapped) = mapper.transform_interval(&unmapped_interval);

                        // Add transformed ones
                        transformed_intervals.append(&mut mapped);

                        // Add unmapped
                        unmapped_intervals.append(&mut unmapped.into_iter().collect::<VecDeque<Interval>>());
                    }
                }
            } else {
                // Add as is to transformed
                transformed_intervals.push(unmapped_interval);
            }

        }

        transformed_intervals
    }
}

fn parse(parts : Vec<&str>) -> (Vec<i64>,Vec<Layer>) {
    let seeds:Vec<i64> = parts[0].split(' ')
        .into_iter()
        .filter(|s| (s.as_bytes()[0] as char).is_digit(10) )
        .map(|s| s.parse().unwrap())
        .collect();

    let mut mappers: Vec<Layer> = vec![];
    for part in parts[1..].into_iter() {
        let lines = part.lines().collect::<VecDeque<&str>>();
        mappers.push(Layer::new(lines));
    }

    (seeds, mappers)
}

fn part1(parts : Vec<&str>) -> String {
    let (seeds, mappers) = parse(parts);
    seeds.iter()
        .map(|seed| {
            let mut next = *seed;
            for mapper in mappers.iter() {
                next = mapper.convert(next);
            }
            next
        })
        .min()
        .unwrap()
        .to_string()

}

fn part2(parts : Vec<&str>) -> String {
    let (seeds, layers) = parse(parts);

    let mut intervals = seeds
        .chunks(2)
        .map(|chunk| Interval{start:chunk[0], end:chunk[0] + chunk[1] - 1})
        .collect::<VecDeque<Interval>>();

    for layer in layers.iter() {
        let mut next_intervals = VecDeque::new();
        while !intervals.is_empty() {
            let interval = intervals.pop_front().unwrap();
            let next = layer.transform(interval);
            next_intervals.append(&mut next.into_iter().collect());
        }

        intervals.append(&mut next_intervals);
    }

    intervals.iter().map(|interval| interval.start).min().unwrap().to_string()
}


#[cfg(test)]
mod tests {
    use std::assert_eq;
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use util::Part::{Part1, Part2};


    const INPUT:&str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    fn test1() {
        assert_eq!("35", solve(INPUT.to_string(), Part1));
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../../input/input_05.txt");
        assert_eq!("331445006", solve(input.to_string(), Part1));
    }

    #[test]
    fn test2() {
        assert_eq!("46", solve(INPUT.to_string(), Part2));
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../../input/input_05.txt");
        assert_eq!("6472060", solve(input.to_string(), Part2));
    }
}
