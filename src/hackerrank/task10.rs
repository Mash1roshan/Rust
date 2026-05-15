pub fn diagonal_difference(arr: Vec<Vec<i32>>) -> i32 {
    let n = arr.len();

    let primary: i32 = (0..n).map(|i| arr[i][i]).sum();
    let secondary: i32 = (0..n).map(|i| arr[i][n - 1 - i]).sum();

    (primary - secondary).abs()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diagonal_difference() {
        let matrix = vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![9, 8, 9],
        ];

        assert_eq!(diagonal_difference(matrix), 2);
    }
}
