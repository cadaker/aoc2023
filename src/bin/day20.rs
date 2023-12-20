use aoc2023::utils::stdio_lines;
use std::collections::HashMap;
use regex::Regex;
use std::collections::VecDeque;

#[derive(Clone)]
enum GateState {
    Broadcast,
    FlipFlop(bool),
    Conjunction(HashMap<String, bool>),
}

#[derive(Clone)]
struct Gate {
    outputs: Vec<String>,
    gate_state: GateState,
}

fn parse_input() -> HashMap<String, Gate> {
    let pattern = Regex::new("([%&]?)([a-z]+) -> (.*)").unwrap();
    let mut nodes = Vec::new();
    let mut inputs: HashMap<&str, Vec<&str>> = HashMap::new();
    let lines = stdio_lines();
    for line in &lines {
        let cap = pattern.captures(&line).unwrap();
        let (_, [tp, name, output_str]) = cap.extract();
        let outputs: Vec<&str> = output_str.split(", ").collect();

        for &out in &outputs {
            if let Some(list) = inputs.get_mut(out) {
                list.push(name);
            } else {
                inputs.insert(out, vec![name]);
            }
        }
        nodes.push((tp, name, outputs));
    }

    let mut ret = HashMap::new();
    for (tp, name, outputs) in nodes {
        let in_nodes: Vec<String> = inputs.get(name).unwrap_or(&Vec::new()).iter().map(|&s| String::from(s)).collect();
        let in_bools = in_nodes.iter().map(|s| (s.clone(), false)).collect();
        ret.insert(
            String::from(name),
            Gate{
                outputs: outputs.iter().map(|&s| String::from(s)).collect(),
                gate_state: match tp {
                    "%" => GateState::FlipFlop(false),
                    "&" => GateState::Conjunction(in_bools),
                    _ => GateState::Broadcast,
                }
            }
        );
    }
    ret
}

type Pulse = bool;

#[derive(Clone)]
struct Message {
    src: String,
    dst: String,
    pulse: Pulse,
}

type Queue = VecDeque<Message>;

fn send_to(outputs: &[String], src: String, pulse: bool) -> Vec<Message> {
    outputs.iter().map(|name| Message{ src: src.clone(), dst: name.clone(), pulse}).collect()
}

fn process(gate: &mut Gate, msg: &Message) -> Vec<Message> {
    match &mut gate.gate_state {
        GateState::Broadcast => {
            send_to(&gate.outputs, msg.dst.clone(), msg.pulse)
        },
        GateState::FlipFlop(onoff) => {
            let onoff = *onoff;
            if msg.pulse {
                vec![]
            } else {
                gate.gate_state = GateState::FlipFlop(!onoff);
                send_to(&gate.outputs, msg.dst.clone(), !onoff)
            }
        },
        GateState::Conjunction(memory) => {
            *memory.get_mut(&msg.src).unwrap() = msg.pulse;
            send_to(&gate.outputs, msg.dst.clone(), !memory.iter().all(|(_, b)| *b))
        }
    }
}

fn run(gates: &mut HashMap<String, Gate>) -> (usize, usize) {
    let mut low = 0;
    let mut high = 0;
    let mut queue = Queue::new();
    queue.push_back(Message{src: String::from("button"), dst: String::from("broadcaster"), pulse: false});

    while !queue.is_empty() {
        let msg = queue.pop_front().unwrap();
        if msg.pulse {
            high += 1;
        } else {
            low += 1;
        }
        if let Some(gate) = gates.get_mut(&msg.dst) {
            for next in process(gate, &msg) {
                queue.push_back(next);
            }
        }
    }

    (low, high)
}

fn main() {
    let mut gates = parse_input();

    let mut low = 0;
    let mut high = 0;
    for _ in 0..1000 {
        let (l, h) = run(&mut gates);
        low += l;
        high += h;
    }
    println!("{}", low * high);
}
