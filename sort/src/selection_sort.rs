pub fn selection_sort(vec: &mut [i64]) {
    for i in 0..vec.len() {
        let mut small = i;
        for j in (i + 1)..vec.len() {
            if vec[j] < vec[small] {
                small = j;
            }
        }
        vec.swap(small, i);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn select_sort_test() {
        let mut test_vec = vec![5, 3, 2, 4, 1];
        let expected_vec = vec![1, 2, 3, 4, 5];

        selection_sort(&mut test_vec);
        assert_eq!(test_vec, expected_vec);
    }

}