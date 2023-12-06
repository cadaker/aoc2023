use std::cmp::max;
use aoc2023::utils::grab_numbers;
use std::io;

fn parse_input() -> (Vec<i64>, Vec<i64>) {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    let times = grab_numbers(&s);
    s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    let distances = grab_numbers(&s);
    (times, distances)
}

fn isqrt_floor(x: i64) -> i64 {
    if x <= 0 {
        return 0
    }
    let mut r = 1;
    while 4 * r * r <= x {
        r *= 2;
    }
    let mut s = r / 2;
    while s > 0 {
        if (r + s) * (r + s) <= x {
            r += s;
        }
        s /= 2;
    }

    return r;
}

fn race(duration: i64, held_time: i64) -> i64 {
    (duration - held_time) * held_time
}

fn count_wins(duration: i64, record: i64) -> i64 {
    // For us to beat the record, the time we hold down the button has to be
    // (D - sqrt(D² - 4R)) / 2 < t < (D + sqrt(D² - 4R)) / 2

    let d = isqrt_floor(duration * duration - 4 * record);

    let mut low = max((duration - d) / 2 - 2, 0);
    let mut high = (duration + d) / 2 + 2;

    while race(duration, low) <= record {
        low += 1;
    }
    while race(duration, high) <= record {
        high -= 1;
    }
    return high - low + 1;
}

fn main() {
    let (times, distances) = parse_input();

    let wins_prod: i64 = times.iter().zip(distances)
        .map(|(duration, record)| count_wins(*duration, record))
        .product();
    println!("{}", wins_prod);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_isqrt_floor() {
        assert_eq!(isqrt_floor(0), 0);
        assert_eq!(isqrt_floor(1), 1);
        assert_eq!(isqrt_floor(2), 1);
        assert_eq!(isqrt_floor(3), 1);
        assert_eq!(isqrt_floor(4), 2);
        assert_eq!(isqrt_floor(5), 2);
        assert_eq!(isqrt_floor(6), 2);
        assert_eq!(isqrt_floor(7), 2);
        assert_eq!(isqrt_floor(8), 2);
        assert_eq!(isqrt_floor(9), 3);
        assert_eq!(isqrt_floor(10), 3);
    }

    #[test]
    fn test_count_wins() {
        assert_eq!(count_wins(7, 9), 4);
        assert_eq!(count_wins(15, 40), 8);
        assert_eq!(count_wins(30, 200), 9);
    }

}
