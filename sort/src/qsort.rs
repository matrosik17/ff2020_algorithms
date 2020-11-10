// TODO: доказать эквивалентность версии с семинара
// TODO2: проанализировать число сравнений и swap'ов
fn partition<T: Ord>(vec: &mut [T], first: usize, last: usize) -> usize {
    let mut i = first;

    for j in first..last {
        if vec[j] < vec[last] {
            if j != i { vec.swap(i, j); }
            i += 1;
        }
    }
    vec.swap(i, last);
    i
}

fn qsort_impl<T: Ord>(array: &mut[T], first: usize, last: usize) {
    if first < last {
        let mid = partition(array, first, last);
        if mid != 0 { qsort_impl(array, first, mid - 1); }
        qsort_impl(array, mid + 1, last);
    }
}

pub fn qsort<T: Ord>(vec: &mut [T]) {
    let first = 0;
    let last = vec.len() - 1;
    qsort_impl(vec, first, last);
}

// TODO: эффективнее использовать стэк
pub fn qsort_iterative<T: Ord>(vec: &mut [T]) {
    let mut stack = Vec::<(usize, usize)>::with_capacity(vec.len());

    let first = 0;
    let last = vec.len() - 1;
    stack.push((first, last));

    while stack.len() != 0 {
        let (first, last) = stack.pop().unwrap();
        let mid = partition(vec, first, last);
        if mid != 0 && first < mid - 1 { stack.push((first, mid - 1)); }
        if mid + 1 < last { stack.push((mid + 1, last)); }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::prelude::*;

    #[test]
    fn qsort_test() {
        let mut test_vec = vec![5, 1, 2, 4, 1];
        let expected_vec = vec![1, 1, 2, 4, 5];

        qsort(&mut test_vec);
        assert_eq!(test_vec, expected_vec);
    }

    #[test]
    fn qsort_rand_test() {
        let n_samples = 100;
        let arr_len = 1000;
        let expected_vec: Vec<i64> = (0..arr_len).map(i64::from).collect();
        let mut rng = thread_rng();

        for _ in 0..n_samples {
            let mut test_vec = expected_vec.clone();
            test_vec.shuffle(&mut rng);

            qsort(&mut test_vec);
            assert_eq!(test_vec, expected_vec);
        }
    }

    #[test]
    fn qsort_iterative_test() {
        let mut test_vec = vec![5, 1, 2, 4, 1];
        let expected_vec = vec![1, 1, 2, 4, 5];

        qsort_iterative(&mut test_vec);
        assert_eq!(test_vec, expected_vec);
    }

    #[test]
    fn qsort_iterative_rand_test() {
        let n_samples = 100;
        let arr_len = 1000;
        let expected_vec: Vec<i64> = (0..arr_len).map(i64::from).collect();
        let mut rng = thread_rng();

        for _ in 0..n_samples {
            let mut test_vec = expected_vec.clone();
            test_vec.shuffle(&mut rng);

            qsort_iterative(&mut test_vec);
            assert_eq!(test_vec, expected_vec);
        }
    }
}