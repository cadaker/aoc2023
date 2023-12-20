pub fn gcd(mut a: usize, mut b: usize) -> usize {
    while a > 0 {
        (a, b) = (b % a, a);
    }
    b
}

pub fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

#[cfg(test)]
mod tests {
    use super::gcd;
    #[test]
    fn test_gcd() {
        assert_eq!(gcd(0, 0), 0);
        assert_eq!(gcd(0, 3), 3);
        assert_eq!(gcd(3, 3), 3);
        assert_eq!(gcd(3, 3), 3);
        assert_eq!(gcd(15, 18), 3);
        assert_eq!(gcd(77, 19), 1);
    }
}
