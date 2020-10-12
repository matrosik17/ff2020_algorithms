pub fn heap_sort(_vec: &mut [i64]) {

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn heap_sort_test() {
        let mut test_vec = vec![5, 3, 2, 4, 1];
        let expected_vec = vec![1, 2, 3, 4, 5];

        heap_sort(&mut test_vec);
        assert_eq!(test_vec, expected_vec);
    }
}