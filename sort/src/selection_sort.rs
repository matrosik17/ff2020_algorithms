pub fn selection_sort<T: Ord>(vec: &mut [T]) {
    for i in 0..vec.len() {
        let mut small = i;
        for j in (i + 1)..vec.len() {
            if vec[j] < vec[small] {
                small = j;
            }
        }
        if i != small { vec.swap(small, i); }
    }
}


pub fn selection_sort_count_swaps<T: Ord>(vec: &mut [T]) -> usize {
    let mut swaps = 0;
    for i in 0..vec.len() {
        let mut small = i;
        for j in (i + 1)..vec.len() {
            if vec[j] < vec[small] {
                small = j;
            }
        }
        if i != small {
            vec.swap(small, i);
            swaps += 1;
        }
    }
    swaps
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::{sort_determenistic_test, sort_random_test};

    #[test]
    fn selection_sort_test() {
        sort_determenistic_test(selection_sort);
    }

    #[test]
    fn selection_sort_rand_test() {
        sort_random_test(selection_sort);
    }

}