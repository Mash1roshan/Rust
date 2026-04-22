pub fn kangaroo(x1: i32, v1: i32, x2: i32, v2: i32) -> String {
    if v1 == v2 {
        if x1 == x2 {
            return "YES".to_string();
        } else {
            return "NO".to_string();
        }
    }

    if (v1 > v2) && (x1 > x2) {
        return "NO".to_string();
    }

    if (v2 > v1) && (x2 > x1) {
        return "NO".to_string();
    }

    if v1 == v2 {
        return "NO".to_string();
    }

    if (x2 - x1) % (v1 - v2) == 0 && v1 > v2 {
        return "YES".to_string();
    }

    "NO".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(kangaroo(0, 3, 4, 2), "YES");
    }

    #[test]
    fn test2() {
        assert_eq!(kangaroo(0, 2, 5, 3), "NO");
    }
}