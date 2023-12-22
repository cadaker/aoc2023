use std::cmp::{min, max};
use aoc2023::utils::{stdio_lines, grab_numbers};
use std::collections::{HashMap, HashSet};

#[derive(Ord, PartialOrd, Eq, PartialEq, Clone, Debug, Hash)]
struct Xyz {
    x: i64,
    y: i64,
    z: i64,
}

impl Xyz {
    fn drop(&self) -> Xyz {
        Xyz{ x: self.x, y: self.y, z: self.z - 1}
    }
}

#[derive(Clone, Debug, Hash)]
struct Brick {
    xyz0: Xyz,
    xyz1: Xyz,
}

fn abs(x: i64) -> i64 {
    if x < 0 { -x } else { x }
}

impl Brick {
    fn new(x0: i64, y0: i64, z0: i64, x1: i64, y1: i64, z1: i64) -> Brick {
        Brick{xyz0: Xyz{ x: x0, y: y0, z: z0 }, xyz1: Xyz{ x: x1, y: y1, z: z1 }}
    }

    fn blocks(&self) -> Vec<Xyz> {
        let [x1, y1, z1] = [self.xyz1.x, self.xyz1.y, self.xyz1.z];
        let mut x = self.xyz0.x;
        let mut y = self.xyz0.y;
        let mut z = self.xyz0.z;
        let [dx, dy, dz] = [x1 - x, y1 - y, z1 - z];
        let n = max(max(abs(dx), abs(dy)), abs(dz));
        let mut ret = Vec::new();
        for _ in 0..n {
            ret.push(Xyz{ x, y, z });
            x += dx / n;
            y += dy / n;
            z += dz / n;
        }
        ret.push(self.xyz1.clone());
        ret
    }
    fn drop(&self) -> Brick {
        Brick{ xyz0: self.xyz0.drop(), xyz1: self.xyz1.drop() }
    }
}

fn parse_input() -> Vec<Brick> {
    let mut ret = Vec::new();
    for line in stdio_lines() {
        let nums = grab_numbers(&line);
        let [x0, y0, z0, x1, y1, z1] = nums[0..6] else { panic!("bad input format") };
        ret.push(Brick::new(x0, y0, z0, x1, y1, z1));
    }
    ret.sort_by_key(|b| min(b.xyz0.z, b.xyz1.z));
    ret
}

fn supports(bricks: &HashMap<Xyz, usize>, next_brick: &Brick) -> HashSet<usize> {
    let mut ret = HashSet::new();
    for block in next_brick.blocks() {
        if let Some(i) = bricks.get(&block.drop()) {
            ret.insert(*i);
        }
    }
    ret
}

fn stack_bricks(bricks: &Vec<Brick>) -> (Vec<Brick>, Vec<HashSet<usize>>, Vec<HashSet<usize>>) {
    let mut new_bricks = Vec::new();
    let mut supporting = Vec::new();
    let mut supported_by = Vec::new();
    let mut placed = HashMap::new();
    for i in 0..bricks.len() {
        supporting.push(HashSet::new());
        let mut brick = bricks[i].clone();
        loop {
            let z = min(brick.xyz0.z, brick.xyz1.z);
            if z == 1 {
                supported_by.push(HashSet::new());
                break;
            }
            let supp = supports(&placed, &brick);
            if supp.is_empty() {
                brick = brick.drop();
            } else {
                for k in supp.iter() {
                    supporting[*k].insert(i);
                }
                supported_by.push(supp);
                break;
            }
        }
        new_bricks.push(brick.clone());
        for block in &brick.blocks() {
            placed.insert(block.clone(), i);
        }
    }
    (new_bricks, supporting, supported_by)
}

fn can_be_disintegrated(i: usize, supporting: &Vec<HashSet<usize>>, supported_by: &Vec<HashSet<usize>>) -> bool {
    for supported in supporting.get(i).unwrap_or(&HashSet::new()).iter() {
        if supported_by.get(*supported).unwrap_or(&HashSet::new()).len() == 1 {
            return false;
        }
    }
    true
}

fn main() {
    let floating_bricks = parse_input();
    let (_flat_bricks, supporting, supported_by) = stack_bricks(&floating_bricks);

    println!("{}", (0..floating_bricks.len()).
        filter(|i| can_be_disintegrated(*i, &supporting, &supported_by))
        .count());
}
