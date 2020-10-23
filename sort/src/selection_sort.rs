pub fn selection_sort<T: Ord>(vec: &mut [T]) {
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
    use rand::prelude::*;

    #[test]
    fn select_sort_test() {
        let mut test_vec = vec![5, 1, 2, 4, 1];
        let expected_vec = vec![1, 1, 2, 4, 5];

        selection_sort(&mut test_vec);
        assert_eq!(test_vec, expected_vec);
    }

    #[test]
    fn select_sort_rand_test() {
        let n_samples = 100;
        let arr_len = 1000;
        let expected_vec: Vec<i64> = (0i64..arr_len).map(i64::from).collect();
        let mut rng = thread_rng();

        for _ in 0..n_samples {
            let mut test_vec = expected_vec.clone();
            test_vec.shuffle(&mut rng);

            selection_sort(&mut test_vec);
            assert_eq!(test_vec, expected_vec);
        }
    }

}