use std::collections::{HashMap, VecDeque};
use super::Part;

pub fn solve(input : String, part: Part) -> String {
    let lines = input.lines().collect();
    match part {
        Part::Part1 => part1(lines),
        Part::Part2 => part2(lines)
    }
}


#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum PulseValue {
    High,
    Low,
}

#[derive(Debug)]
struct Pulse {
    src:String,
    pulse_value:PulseValue,
    dst:String,
}

impl Pulse {
    fn new(pulse_value:PulseValue, src:String, dst:String) -> Pulse {
        Pulse{pulse_value, src, dst}
    }

    fn desc(&self) -> String {
        format!("{} -{:?}-> {}", self.src, self.pulse_value, self.dst)
    }
}



trait Processor {
    // Takes a pulse as input and produces output
    fn process(&mut self, pulse:Pulse) -> Vec<Pulse>;
    fn get_name(&self) -> &String;

    fn get_outputs(&self) -> &Vec<String>;

    fn add_input(&mut self, input:&String);
}


#[derive(Debug)]
struct BroadCaster {
    name:String,
    outputs:Vec<String>,
}

#[derive(Debug)]
struct FlipFlop {
    mode:bool,
    name:String,
    outputs:Vec<String>,
}

#[derive(Debug)]
struct Conjunction {
    inputs:Vec<(String, PulseValue)>,
    name:String,
    outputs:Vec<String>,
}

#[derive(Debug)]
struct OutputNode {
    name:String,
    outputs:Vec<String>,
}

impl OutputNode {
    fn new(name:&str) -> OutputNode {
        OutputNode{name:name.to_string(), outputs:vec![]}
    }
}

impl Processor for OutputNode {
    fn process(&mut self, _pulse: Pulse) -> Vec<Pulse> {
        vec![]
    }

    fn get_name(&self) -> &String {
        &self.name
    }

    fn get_outputs(&self) -> &Vec<String> {
        &self.outputs
    }

    fn add_input(&mut self, _input: &String) {
    }
}

impl BroadCaster {
    fn new(name:&str, outputs:Vec<&str>) -> BroadCaster {
        BroadCaster{name:name.to_string(), outputs:outputs.iter().map(|s| s.to_string()).collect()}
    }
}

impl FlipFlop {
    fn new(name:&str, outputs:Vec<&str>) -> FlipFlop {
        FlipFlop{mode:false, name:name.to_string(), outputs:outputs.iter().map(|s| s.to_string()).collect()}
    }
}

impl Conjunction {
    fn new(name:&str, outputs:Vec<&str>) -> Conjunction {
        Conjunction{inputs:vec![], name:name.to_string(), outputs:outputs.iter().map(|s| s.to_string()).collect()}
    }
}

impl Processor for  BroadCaster {
    fn process(&mut self, _pulse: Pulse) -> Vec<Pulse> {
        self.outputs.iter()
            .map(|output| Pulse::new(PulseValue::Low, self.get_name().clone(), output.clone()))
            .collect()
    }
    fn get_name(&self) -> &String {
        &self.name
    }
    fn get_outputs(&self) -> &Vec<String> {
        &self.outputs
    }

    fn add_input(&mut self, _input: &String) {

    }
}

impl Processor for  Conjunction {
    fn process(&mut self, pulse: Pulse) -> Vec<Pulse> {

        let item = self.inputs.iter_mut().find(|item| item.0.eq(&pulse.src)).unwrap();
        item.1 = pulse.pulse_value;

        let all_high = self.inputs.iter().all(|item| item.1 == PulseValue::High);
        let next_pulse = if all_high {
            PulseValue::Low
        } else {
            PulseValue::High
        };

        self.outputs.iter()
            .map(|output| Pulse::new(next_pulse, self.get_name().clone(), output.clone()))
            .collect()
    }
    fn get_name(&self) -> &String {
        &self.name
    }

    fn get_outputs(&self) -> &Vec<String> {
        &self.outputs
    }

    fn add_input(&mut self, input:&String) {
        self.inputs.push((input.clone(), PulseValue::Low))
    }
}

impl Processor for FlipFlop {
    fn process(&mut self, pulse: Pulse) -> Vec<Pulse> {
        if pulse.pulse_value == PulseValue::Low {
            self.mode = !self.mode;
            let next_pulse = match self.mode {
                true => PulseValue::High,
                false => PulseValue::Low,
            };
            self.outputs.iter()
                .map(|output| Pulse::new(next_pulse, self.get_name().clone(), output.clone()))
                .collect()
        } else {
            vec![]
        }
    }
    fn get_name(&self) -> &String {
        &self.name
    }
    fn get_outputs(&self) -> &Vec<String> {
        &self.outputs
    }

    fn add_input(&mut self, _in:&String) {

    }
}


fn parse_line(line:&str) -> Box<dyn Processor> {
    let mut items:Vec<&str> = line.split(|c| c == ' ' || c == ',')
        .filter(|s| !s.is_empty() && !s.eq(&"->"))
        .collect();

    let module_desc = items.remove(0);
    let name = &module_desc[1..];
    let outputs = items;
    if module_desc.starts_with("broadcaster") {
        Box::new(BroadCaster::new(module_desc, outputs))
    } else if module_desc.starts_with("%") {
        Box::new(FlipFlop::new(name, outputs))

    } else if module_desc.starts_with("&") {
        Box::new(Conjunction::new(name, outputs))
    } else {
        panic!("...")
    }
}


fn build_modules(lines : Vec<&str>) -> Vec<Box<dyn Processor>> {
    let mut modules:Vec<Box< dyn Processor>> = lines.iter()
        .map(|line| parse_line(line)).collect();
    let output_pairs = modules.iter()
        .map(|m | (m.get_name().clone(), m.get_outputs().clone()))
        .collect::<Vec<(String, Vec<String>)>>();

    let outputs:Vec<&String> = output_pairs.iter().map(|(_name, outs)| outs).flatten().collect();
    let output_name = outputs.iter().find(|output| modules.iter().all(|m| !m.get_name().as_str().eq(output.as_str()))).unwrap();
    let output_node = OutputNode::new(output_name.as_str());
    modules.push(Box::new(output_node));


    for (src, outputs) in output_pairs.iter() {
        for out in outputs {
            let module = modules.iter_mut()
                .find(|m| m.get_name().as_str().eq(out.as_str())).unwrap();
            module.add_input(&src);
        }
    }
    modules
}

fn get_parent_node(node_name:&str, modules:&Vec<Box<dyn Processor>>) -> String {
    for module in modules.iter() {
        if module.get_outputs().iter().any(|out| out.as_str().eq(node_name)) {
            return module.get_name().to_string();
        }
    }

    panic!("...")
}

fn part1(lines : Vec<&str>) -> String {
    let mut modules = build_modules(lines);
    let mut queue = VecDeque::new();
    let mut high_pulses = 0;
    let mut low_pulses = 0;

    for _ in 0..1000 {
        queue.push_front(Pulse::new(PulseValue::Low, String::from("aptly"), String::from("broadcaster")));

        // Run
        while !queue.is_empty() {
            let pulse = queue.pop_front().unwrap();

            if pulse.pulse_value == PulseValue::High {
                high_pulses += 1;
            } else {
                low_pulses += 1;
            }

            // Find destination module
            let module = modules.iter_mut().find(|m| m.get_name().eq(&pulse.dst)).unwrap();

            module.process(pulse).into_iter()
                .for_each(|out_pulse| queue.push_back(out_pulse));

        }
    }

    (low_pulses * high_pulses).to_string()
}

fn part2(lines : Vec<&str>) -> String {
    let mut modules = build_modules(lines);
    let mut queue = VecDeque::new();
    let parent_node = get_parent_node("rx", &modules);
    let mut parent_inputs = HashMap::new();
    println!("parent node:{}", parent_node);

    for i in 1..5000 {
        queue.push_front(Pulse::new(PulseValue::Low, String::from("aptly"), String::from("broadcaster")));

        // Run
        while !queue.is_empty() {
            let pulse = queue.pop_front().unwrap();

            if pulse.dst.as_str().eq(parent_node.as_str()) {
                if pulse.pulse_value == PulseValue::High {
                    if !parent_inputs.contains_key(&pulse.src) {
                        parent_inputs.insert(pulse.src.to_string(), i as u64);
                        println!("i={}, {}", i, pulse.desc());
                    }
                }
            }

            let module = modules.iter_mut().find(|m| m.get_name().eq(&pulse.dst)).unwrap();
            let pulses = module.process(pulse);
            for p in pulses {
                queue.push_back(p);
            }
        }
    }

    parent_inputs.values().product::<u64>().to_string()
}


#[cfg(test)]
mod tests {
    use std::assert_eq;
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use util::Part::{Part1, Part2};


    const INPUT:&str = "broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output";


    #[test]
    fn test1() {
        assert_eq!("11687500", solve(INPUT.to_string(), Part1));
    }



    #[test]
    fn test_part1() {
        let input = include_str!("../../input/input_20.txt");
        assert_eq!("898731036", solve(input.to_string(), Part1));
    }



    #[test]
    fn test_part2() {
        let input = include_str!("../../input/input_20.txt");
        assert_eq!("229414480926893", solve(input.to_string(), Part2));
    }
}
