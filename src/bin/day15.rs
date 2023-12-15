use aoc2023::utils::stdio_lines;

fn parse_input() -> Vec<String> {
    let mut ret = Vec::new();
    for line in stdio_lines() {
        for part in line.split(",") {
            ret.push(String::from(part));
        }
    }
    ret
}

fn hash(s: &str) -> i64 {
    let mut h = 0;
    for ch in s.chars() {
        h = ((h + ch as i64) * 17) % 256
    }
    h
}

fn set_lens(boxes: &mut [Vec<(String, i64)>], label: &str, val: i64) {
    let boxno = hash(label);
    let lenses = &mut boxes[boxno as usize];
    for (i, (lbl, _)) in lenses.iter().enumerate() {
        if lbl == label {
            lenses[i] = (String::from(label), val);
            return;
        }
    }
    lenses.push((String::from(label), val));
}

fn remove_lens(boxes: &mut [Vec<(String, i64)>], label: &str) {
    let boxno = hash(label);
    let lenses = &mut boxes[boxno as usize];
    for (i, (lbl, _)) in lenses.iter().enumerate() {
        if lbl == label {
            lenses.remove(i);
            return;
        }
    }
}

fn execute(boxes: &mut [Vec<(String, i64)>], oper: &str) {
    if oper.ends_with("-") {
        remove_lens(boxes, &oper[0..oper.len()-1]);
    } else {
        let parts: Vec<&str> = oper.split("=").collect();
        let (label, lens) = (parts[0], parts[1]);
        set_lens(boxes, label, lens.parse().unwrap());
    }
}

fn focusing_power(boxes: &Vec<(String, i64)>, boxno: i64) -> i64 {
    boxes.iter()
        .enumerate()
        .map(|(i, (_, val))| (boxno + 1) * (i as i64 + 1) * val)
        .sum::<i64>()
}

fn main() {
    let input = parse_input();
    println!("{}",
        input.iter()
            .map(|s| hash(s.as_str()))
            .sum::<i64>()
    );

    let mut boxes = Vec::new();
    boxes.resize(256, Vec::new());
    for oper in &input {
        execute(&mut boxes, oper);
    }

    println!("{}", boxes.iter()
        .enumerate()
        .map(|(i, b)| focusing_power(b, i as i64))
        .sum::<i64>()
    )
}
