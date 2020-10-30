fn sift<T: Ord>(vec: &mut [T], left_idx: usize, right_idx: usize) {
    let mut root_idx = left_idx;
    loop {
        // get child indices
        let left_child_idx = 2 * root_idx + 1;
        let right_child_idx = left_child_idx + 1;

        // check heap bounds
        if left_child_idx >= right_idx { break; }

        // get child elements
        let left_child = vec.get(left_child_idx);
        let mut right_child = vec.get(right_child_idx);
        if right_child_idx >= right_idx { right_child = None; }

        let child_largest_idx = if left_child < right_child {
            right_child_idx
        } else {
            left_child_idx
        };

        // check max_heap invariant
        if vec[root_idx] < vec[child_largest_idx] {
            vec.swap(root_idx, child_largest_idx);
            root_idx = child_largest_idx;
        } else {
            break;
        }
    }
}

pub fn heap_sort<T: Ord>(vec: &mut [T]) {
    // build heap
    for root_idx in (0..vec.len()).rev() {
        sift(vec, root_idx, vec.len());
    }

    // sort
    for right_idx in (1..vec.len()).rev() {
        vec.swap(0, right_idx);
        sift(vec, 0, right_idx);
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use rand::prelude::*;

    #[test]
    fn heap_sort_test() {
        let mut test_vec = vec![5, 1, 2, 4, 1];
        let expected_vec = vec![1, 1, 2, 4, 5];

        heap_sort(&mut test_vec);
        assert_eq!(test_vec, expected_vec);
    }

    #[test]
    fn heap_sort_rand_test() {
        let n_samples = 100;
        let arr_len = 1000;
        let expected_vec: Vec<i64> = (0..arr_len).map(i64::from).collect();
        let mut rng = thread_rng();

        for _ in 0..n_samples {
            let mut test_vec = expected_vec.clone();
            test_vec.shuffle(&mut rng);

            heap_sort(&mut test_vec);
            assert_eq!(test_vec, expected_vec);
        }
    }
}