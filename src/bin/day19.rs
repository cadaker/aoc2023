use regex::Regex;
use std::collections::HashMap;
use aoc2023::utils::stdio_lines;

#[derive(Eq, PartialEq)]
enum Decision {
    Accept,
    Reject,
    Forward(String),
}

#[derive(Eq, PartialEq)]
enum Op {
    LT,
    GT,
}

struct BranchRule {
    var: String,
    op: Op,
    limit: i64,
    decision: Decision,
}

enum Rule {
    Branch(BranchRule),
    Finish(Decision),
}

struct Workflow {
    name: String,
    rules: Vec<Rule>,
}

#[derive(Clone)]
struct Part {
    x: i64,
    m: i64,
    a: i64,
    s: i64,
}

fn parse_decision(s: &str) -> Decision {
    if s == "A" {
        Decision::Accept
    } else if s == "R" {
        Decision::Reject
    } else {
        Decision::Forward(String::from(s))
    }
}

fn parse_workflow(line: &str) -> Workflow {
    let workflow_pattern = Regex::new("([a-zA-Z]+)\\{(.*)}").unwrap();
    let rule_pattern = Regex::new("([xmas])([<>])([-0-9]+):([A-Za-z]+)").unwrap();

    let cap = workflow_pattern.captures(&line).unwrap();
    let [name, rulestr] = cap.extract().1;
    let mut rules = Vec::new();
    for part in rulestr.split(',') {
        let cap = rule_pattern.captures(part);
        if cap.is_some() {
            let [var, op, limit, decision] = cap.unwrap().extract().1;
            let rule = BranchRule {
                var: String::from(var),
                op: match op {
                    "<" => Op::LT,
                    ">" => Op::GT,
                    _ => panic!("Bad op")
                },
                limit: limit.parse().unwrap(),
                decision: parse_decision(decision),
            };
            rules.push(Rule::Branch(rule));
        } else {
            rules.push(Rule::Finish(parse_decision(part)));
        }
    }
    Workflow{ name: String::from(name), rules }
}

fn parse_part(line: &str) -> Part {
    let pattern = Regex::new("\\{x=([-0-9]+),m=([-0-9]+),a=([-0-9]+),s=([-0-9]+)}").unwrap();
    let cap = pattern.captures(line).unwrap();
    let [x,m,a,s] = cap.extract().1;
    Part {
        x: x.parse().unwrap(),
        m: m.parse().unwrap(),
        a: a.parse().unwrap(),
        s: s.parse().unwrap(),
    }
}

fn parse_input() -> (HashMap<String, Workflow>, Vec<Part>) {
    let mut workflows = HashMap::new();

    let lines = stdio_lines();
    for i in 0..lines.len() {
        if lines[i].len() == 0 {
            break;
        }
        let workflow = parse_workflow(&lines[i]);
        workflows.insert(workflow.name.clone(), workflow);
    }

    let mut parts = Vec::new();
    for line in &lines[workflows.len() + 1..] {
        parts.push(parse_part(line));
    }

    (workflows, parts)
}

fn resolve(workflows: &HashMap<String, Workflow>, part: &Part) -> Decision {
    let mut flow = String::from("in");
    loop {
        let workflow = workflows.get(&flow).unwrap();
        for rule in &workflow.rules {
            if let Rule::Finish(Decision::Accept) = rule {
                return Decision::Accept;
            } else if let Rule::Finish(Decision::Reject) = rule {
                return Decision::Reject;
            } else if let Rule::Finish(Decision::Forward(next)) = rule {
                flow = next.clone();
                break;
            } else if let Rule::Branch(branch) = rule {
                let val = match branch.var.as_str() {
                    "x" => part.x,
                    "m" => part.m,
                    "a" => part.a,
                    "s" => part.s,
                    _ => panic!("Invalid var"),
                };
                let matches = if branch.op == Op::LT {
                    val < branch.limit
                } else {
                    val > branch.limit
                };
                if matches {
                    match &branch.decision {
                        Decision::Accept => return Decision::Accept,
                        Decision::Reject => return Decision::Reject,
                        Decision::Forward(next) => {
                            flow = next.clone();
                            break;
                        }
                    }
                }
            } else {
                panic!("No matching case");
            }
        }
    }
}

fn main() {
    let (workflows, parts) = parse_input();

    let accepted: Vec<Part> = parts.iter()
        .filter(|&p| resolve(&workflows, p) == Decision::Accept)
        .cloned()
        .collect();
    println!("{}", accepted.iter().map(|p| p.x + p.m + p.a + p.s).sum::<i64>());
}
