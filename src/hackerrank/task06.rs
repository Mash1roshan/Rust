pub fn get_total_x(a: &[i32], b: &[i32]) -> i32 {
    fn gcd(mut x: i32, mut y: i32) -> i32 {
        while y != 0 {
            let temp = y;
            y = x % y;
            x = temp;
        }
        x
    }

    fn lcm(x: i32, y: i32) -> i32 {
        x / gcd(x, y) * y
    }

    let lcm_a = a.iter().copied().reduce(lcm).unwrap();
    let gcd_b = b.iter().copied().reduce(gcd).unwrap();

    let mut count = 0;
    let mut current = lcm_a;

    while current <= gcd_b {
        if gcd_b % current == 0 {
            count += 1;
        }
        current += lcm_a;
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_case() {
        let a = vec![2, 4];
        let b = vec![16, 32, 96];
        assert_eq!(get_total_x(&a, &b), 3);
    }

    #[test]
    fn test_single_element() {
        let a = vec![3];
        let b = vec![9];
        assert_eq!(get_total_x(&a, &b), 2);
    }

    #[test]
    fn test_another_case() {
        let a = vec![2];
        let b = vec![20, 30, 12];
        assert_eq!(get_total_x(&a, &b), 1);
    }
}