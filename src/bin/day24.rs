use aoc2023::utils::{stdio_lines, grab_numbers};
use num_rational::{Rational64, BigRational};
use num_bigint::BigInt;
use num_traits::cast::FromPrimitive;

#[derive(Debug)]
struct Vector {
    x: i64,
    y: i64,
    z: i64,
}

fn parse_input() -> Vec<(Vector, Vector)> {
    stdio_lines().iter()
        .map(|line| grab_numbers(line))
        .map(|nums| {
            let pos = Vector{ x: nums[0], y: nums[1], z: nums[2] };
            let vel = Vector{ x: nums[3], y: nums[4], z: nums[5] };
            (pos, vel)
        })
        .collect()
}

#[derive(Debug)]
enum Collide {
    Never,
    Always,
    At(Rational64)
}

fn intersection_xy(p0: &Vector, v0: &Vector, p1: &Vector, v1: &Vector) -> Collide {
    // We have a system of equations
    // [vx  -vx'] [t ]  =  [x' - x]
    // [vy  -vy'] [t']  =  [y' - y]
    let det = (v0.x * -v1.y) - (-v1.x * v0.y);
    let dx = p1.x - p0.x;
    let dy = p1.y - p0.y;
    if det != 0 {
        let t0 = Rational64::new(dx * -v1.y - (dy * -v1.x), det);
        let t1 = Rational64::new(v0.x * dy - v0.y * dx, det);
        if t0 >= Rational64::from_integer(0) && t1 >= Rational64::from_integer(0) {
            Collide::At(t0)
        } else {
            Collide::Never
        }
    } else {
        // Linearly dependent. We'll assume that they never overlap.
        Collide::Never
    }
}

fn to_bigint(r: &Rational64) -> BigRational {
    BigRational::new(BigInt::from_i64(*r.numer()).unwrap(), BigInt::from_i64(*r.denom()).unwrap())
}

fn collides_inside_xy((p0, v0): &(Vector, Vector), c: &Collide, min: i64, max: i64) -> bool {
    let min = to_bigint(&Rational64::from_integer(min));
    let max = to_bigint(&Rational64::from_integer(max));
    match c {
        Collide::Never => false,
        Collide::Always => true,
        Collide::At(t) => {
            if *t >= Rational64::from_integer(0) {
                let t = to_bigint(t);
                let x = BigRational::from_i64(p0.x).unwrap() + t.clone() * BigRational::from_i64(v0.x).unwrap();
                let y = BigRational::from_i64(p0.y).unwrap() + t * BigRational::from_i64(v0.y).unwrap();
                min <= x && x <= max && min <= y && y <= max
            } else {
                false
            }
        }
    }
}

fn main() {
    let hailstones = parse_input();
    const MIN: i64 = 200000000000000;
    const MAX: i64 = 400000000000000;

    let mut count = 0usize;

    for i in 0..hailstones.len() {
        for j in i + 1..hailstones.len() {
            let coll = intersection_xy(&hailstones[i].0, &hailstones[i].1, &hailstones[j].0, &hailstones[j].1);
            if collides_inside_xy(&hailstones[i], &coll, MIN, MAX) {
            // if collides_inside_xy(&hailstones[i], &coll, 7, 27) {
                count += 1;
            }
        }
    }
    println!("{}", count);
}
