use regex::Regex;
use std::collections::HashMap;
use aoc2023::utils::stdio_lines;

#[derive(Eq, PartialEq)]
enum Decision {
    Accept,
    Reject,
    Forward(String),
}

#[derive(Eq, PartialEq, Copy, Clone)]
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

#[derive(Clone)]
struct PartsRange {
    minx: i64,
    maxx: i64,
    minm: i64,
    maxm: i64,
    mina: i64,
    maxa: i64,
    mins: i64,
    maxs: i64,
}

fn empty(range: &PartsRange) -> bool {
    !(range.minx <= range.maxx && range.minm <= range.maxm && range.mina <= range.maxa && range.mins <= range.maxs)
}

fn cut_range(range: &PartsRange, var: &str, op: Op, limit: i64) -> (PartsRange, PartsRange) {
    let PartsRange {minx, maxx, minm, maxm, mina, maxa, mins, maxs} = *range;
    match (var, op) {
        ("x", Op::LT) => (PartsRange{ minx, maxx: limit - 1, minm, maxm, mina, maxa, mins, maxs },
                          PartsRange{ minx: limit, maxx, minm, maxm, mina, maxa, mins, maxs }),
        ("x", Op::GT) => (PartsRange{ minx: limit + 1, maxx, minm, maxm, mina, maxa, mins, maxs },
                          PartsRange{ minx, maxx: limit, minm, maxm, mina, maxa, mins, maxs }),
        ("m", Op::LT) => (PartsRange{ minx, maxx, minm, maxm: limit - 1, mina, maxa, mins, maxs },
                          PartsRange{ minx, maxx, minm: limit, maxm, mina, maxa, mins, maxs }),
        ("m", Op::GT) => (PartsRange{ minx, maxx, minm: limit + 1, maxm, mina, maxa, mins, maxs },
                          PartsRange{ minx, maxx, minm, maxm: limit, mina, maxa, mins, maxs }),
        ("a", Op::LT) => (PartsRange{ minx, maxx, minm, maxm, mina, maxa: limit - 1, mins, maxs },
                          PartsRange{ minx, maxx, minm, maxm, mina: limit, maxa, mins, maxs }),
        ("a", Op::GT) => (PartsRange{ minx, maxx, minm, maxm, mina: limit + 1, maxa, mins, maxs },
                          PartsRange{ minx, maxx, minm, maxm, mina, maxa: limit, mins, maxs }),
        ("s", Op::LT) => (PartsRange{ minx, maxx, minm, maxm, mina, maxa, mins, maxs: limit - 1 },
                          PartsRange{ minx, maxx, minm, maxm, mina, maxa, mins: limit, maxs }),
        ("s", Op::GT) => (PartsRange{ minx, maxx, minm, maxm, mina, maxa, mins: limit + 1, maxs },
                          PartsRange{ minx, maxx, minm, maxm, mina, maxa, mins, maxs: limit }),
        _ => panic!("Bad range cut"),
    }
}

fn combination_count(range: &PartsRange) -> i64 {
    (range.maxx - range.minx + 1) *
        (range.maxm - range.minm + 1) *
        (range.maxa - range.mina + 1) *
        (range.maxs - range.mins + 1)
}

fn combinations(workflows: &HashMap<String, Workflow>, flow: &String, range: &PartsRange) -> i64 {
    let mut remaining_range = range.clone();

    let workflow = workflows.get(flow).unwrap();
    let mut ret = 0;
    for rule in &workflow.rules {
        if let Rule::Finish(Decision::Accept) = rule {
            ret += combination_count(&remaining_range);
            break;
        } else if let Rule::Finish(Decision::Reject) = rule {
            break;
        } else if let Rule::Finish(Decision::Forward(next)) = rule {
            ret += combinations(workflows, next, &remaining_range);
            break;
        } else if let Rule::Branch(BranchRule{ var, op, limit, decision}) = rule {
            let (true_branch, false_branch) = cut_range(&remaining_range, var, *op, *limit);
            if !empty(&true_branch) {
                if let Decision::Accept = decision {
                    ret += combination_count(&true_branch);
                } else if let Decision::Forward(next) = decision {
                    ret += combinations(workflows, next, &true_branch);
                }
            }
            if empty(&false_branch) {
                break;
            }
            remaining_range = false_branch;
        } else {
            panic!("Bad rule");
        }
    }
    ret
}

fn main() {
    let (workflows, parts) = parse_input();

    let accepted: Vec<Part> = parts.iter()
        .filter(|&p| resolve(&workflows, p) == Decision::Accept)
        .cloned()
        .collect();
    println!("{}", accepted.iter().map(|p| p.x + p.m + p.a + p.s).sum::<i64>());

    let range = PartsRange{
        minx: 1, maxx: 4000,
        minm: 1, maxm: 4000,
        mina: 1, maxa: 4000,
        mins: 1, maxs: 4000,
    };
    println!("{}", combinations(&workflows, &String::from("in"), &range));
}
