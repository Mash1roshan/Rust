use std::collections::HashMap;

pub fn sock_merchant(ar: &[i32]) -> i32 {
    let mut map = HashMap::new();

    for &sock in ar {
        *map.entry(sock).or_insert(0) += 1;
    }

    map.values().map(|count| count / 2).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_case() {
        let arr = vec![1, 2, 1, 2, 1, 3, 2];
        assert_eq!(sock_merchant(&arr), 2);
    }

    #[test]
    fn test_all_pairs() {
        let arr = vec![10, 10, 20, 20, 30, 30];
        assert_eq!(sock_merchant(&arr), 3);
    }

    #[test]
    fn test_no_pairs() {
        let arr = vec![1, 2, 3, 4];
        assert_eq!(sock_merchant(&arr), 0);
    }

    #[test]
    fn test_empty() {
        let arr: Vec<i32> = vec![];
        assert_eq!(sock_merchant(&arr), 0);
    }
}