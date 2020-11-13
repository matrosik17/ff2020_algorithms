pub fn selection_sort<T: Ord + Copy>(vec: &mut [T]) {
    for i in 0..vec.len() {
        let mut small = i;
        for j in (i + 1)..vec.len() {
            if vec[j] < vec[small] {
                small = j;
            }
        }
        let tmp = vec[i];
        vec[i] = vec[small];
        vec[small] = tmp;
        // vec.swap(small, i);
        // TODO: заменить swap
        // TODO: добавить таймер
    }
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