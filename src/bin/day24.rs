use aoc2023::utils::{stdio_lines, grab_numbers};
use num_rational::{Rational64, BigRational};
use num_bigint::BigInt;
use num_traits::cast::FromPrimitive;
use num_traits::ToPrimitive;

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

// For part 2:
// We have variables p (vector), v (vector), t_i (scalars) such that
//   p + t_i * v = p_i + t_i * v_i for all i.

// For the 3d case, we have n + 6 variables, and 3n equations. This will make the system
// overdetermined if n > 3. That is, we can pick just n == 3 to get a perfectly determined
// system of equations, and assume that the solution will fit with the rest of the overdetermined
// system.

// This gives us the system
//   p + t1 * v = p1 + t1 * v1
//   p + t2 * v = p2 + t2 * v2
//   p + t3 * v = p3 + t3 * v3
// We can first eliminate p from the bottom two equations. This leaves us with
//   (t2 - t1) * v = p2 - p1 + t2*v2 - t1*v1
//   (t3 - t1) * v = p3 - p1 + t3*v3 - t1*v1
// Then proceed to eliminate v from the third equation. This will leave us with a vector equation
//   (t1*v1 - t3*v3)(t2 - t1) - (t1*v1 - t2*v2)(t3 - t1) = (p3 - p1)(t2 - t1) - (p2 - p1)(t3 - t1)
// or
//   t1*t2(v1-v2) + t2*t3*(v2-v3) + t1*t3*(v3-v1) = t1*(p2-p3) + t2*(p3-p1) + t3*(p1-p2)

// Introduce a1 = v1-v2, a2 = v2-v3, b1 = p2-p3, b2 = p3-p1. Then we have
//   t1*t2*a1 + t2*t3*a2 - t1*t3*(a1+a2) = t1*b1 + t2*b2 - t3*(b1 + b2)

// This can then be factored as
//   (t1*a1 - b2)*(t2 - t3) + t3*a2*(t2 - t1) + b1*(t1 - t3) = 0
// Breaking apart by component now lets us solve for the different ts.
//   t2 = (t3*(t1*a1x - b2x) + t3*t1*a2x + (t1 - t3)*b1x) / (t1*a1x - b2x + t3*a2x)
//   t3 = (-b1x*(t1*a1y - b2y) + b1y*(t1*a1x - b2x)) / (a2x*(t1*a1y - b1y - b2y) - a2y*(t1*a1x - b1x - b2x ))
// Introducting new variables
//   Tx = (a1x*t1 - b2x), Ty = (a1y*t1 - b2y), Tz = (ayz*t1 - b2z)
// as well as
//   Dx = a2y*b1z - a2z*b1y; Dy = a2z*b1x - a2x*b1z; Dz = a2x*b1y - a2y*b1z;
// eventually yields (Dx*Tx + Dy*Ty + Dz*Tz)(Tx - b1x) = 0
// The solution Tx = b1x turns out to be invalid, so we have
//   Dx*Tx + Dy*Ty + Dz*Tz = 0, or
//   t1 = (b2x*Dx + b2y*Dy + b2z*Dz) / (a1x*Dx + a1y*Dy + a1z*Dz)
// With t1, t2, and t3, we can now solve for v, and later for p.

fn solve_intersection(p1: &Vector, v1: &Vector, p2: &Vector, v2: &Vector, p3: &Vector, v3: &Vector) -> Vector {
    let Vector{x: p1x, y: p1y, z: p1z} = p1;
    let Vector{x: p2x, y: p2y, z: p2z} = p2;
    let Vector{x: p3x, y: p3y, z: p3z} = p3;
    let Vector{x: v1x, y: v1y, z: v1z} = v1;
    let Vector{x: v2x, y: v2y, z: v2z} = v2;
    let Vector{x: v3x, y: v3y, z: v3z} = v3;

    let big = |x| { BigInt::from_i64(x).unwrap() };
    let rat = |x| { BigRational::from_integer(x) };
    let bigrat = |x| { rat(big(x)) };

    let (a1x, a1y, a1z) = (big(v1x - v2x), big(v1y - v2y), big(v1z - v2z));
    let (a2x, a2y, a2z) = (big(v2x - v3x), big(v2y - v3y), big(v2z - v3z));
    let (b1x, b1y, b1z) = (big(p2x - p3x), big(p2y - p3y), big(p2z - p3z));
    let (b2x, b2y, b2z) = (big(p3x - p1x), big(p3y - p1y), big(p3z - p1z));

    let dx = a2y.clone() * b1z.clone() - a2z.clone() * b1y.clone();
    let dy = a2z.clone() * b1x.clone() - a2x.clone() * b1z.clone();
    let dz = a2x.clone() * b1y.clone() - a2y.clone() * b1x.clone();

    let t1 = BigRational::new(
        b2x.clone() * dx.clone() + b2y.clone() * dy.clone() + b2z * dz.clone(),
        a1x.clone() * dx + a1y.clone() * dy + a1z * dz);
    let t1x = t1.clone() * a1x - b2x;
    let t1y = t1.clone() * a1y - b2y;
    let t3 = (-rat(b1x.clone()) * t1y.clone() + rat(b1y.clone()) * t1x.clone()) / (rat(a2x.clone()) * (t1y - b1y) - rat(a2y) * (t1x.clone() - b1x.clone()));

    let t2 = (t3.clone() * t1x.clone() + t1.clone() * t3.clone() * a2x.clone() + (t1.clone() - t3.clone()) * b1x) / (t1x + t3.clone() * a2x);


    let vx = (bigrat(*p2x - *p1x) + t2.clone() * bigrat(*v2x) - t1.clone() * bigrat(*v1x)) / (t2.clone() - t1.clone());
    let vy = (bigrat(*p2y - *p1y) + t2.clone() * bigrat(*v2y) - t1.clone() * bigrat(*v1y)) / (t2.clone() - t1.clone());
    let vz = (bigrat(*p2z - *p1z) + t2.clone() * bigrat(*v2z) - t1.clone() * bigrat(*v1z)) / (t2.clone() - t1.clone());

    let px = bigrat(*p1x) + t1.clone() * (bigrat(*v1x) - vx.clone());
    let py = bigrat(*p1y) + t1.clone() * (bigrat(*v1y) - vy.clone());
    let pz = bigrat(*p1z) + t1.clone() * (bigrat(*v1z) - vz.clone());

    assert!(px.is_integer());
    assert!(py.is_integer());
    assert!(pz.is_integer());
    Vector { x: px.to_i64().unwrap(), y: py.to_i64().unwrap(), z: pz.to_i64().unwrap() }
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
    let p = solve_intersection(
        &hailstones[0].0,
        &hailstones[0].1,
        &hailstones[1].0,
        &hailstones[1].1,
        &hailstones[2].0,
        &hailstones[2].1);
    println!("{}", p.x + p.y + p.z);
}
