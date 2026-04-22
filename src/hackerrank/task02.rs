pub fn staircase(n: usize) -> Vec<String> {
    (1..=n)
        .map(|i| {
            let spaces = " ".repeat(n - i);
            let hashes = "#".repeat(i);
            format!("{}{}", spaces, hashes)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_staircase() {
        let result = staircase(4);
        let expected = vec![
            "   #",
            "  ##",
            " ###",
            "####"
        ];
        assert_eq!(result, expected);
    }
}