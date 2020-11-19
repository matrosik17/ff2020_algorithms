pub fn shell_sort<T: Ord>(vec: &mut [T]) {
    let len = vec.len();
    let mut gap = len / 2;
    while gap > 0 {
        for i in gap..len {
            for j in (0..=i).rev().step_by(gap).skip(1) {
                if vec[j] <= vec[j + gap] { continue; }
                vec.swap(j, j + gap);
            }
        }
        gap /= 2;
    }
}

pub fn shell_sort_knuth<T: Ord>(vec: &mut [T]) {
    let len = vec.len();
    let mut gap = 1;
    while gap < len / 3 { gap = 2 * gap + 1; }

    while gap >= 1 {
        for i in gap..len {
            for j in (0..=i).rev().step_by(gap).skip(1) {
                if vec[j] <= vec[j + gap] { continue; }
                vec.swap(j, j + gap);
            }
        }
        gap /= 2;
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::{sort_determenistic_test, sort_random_test};

    #[test]
    fn shell_sort_test() {
        sort_determenistic_test(shell_sort);
    }

    #[test]
    fn shell_sort_rand_test() {
        sort_random_test(shell_sort);
    }

    #[test]
    fn shell_sort_knuth_test() {
        sort_determenistic_test(shell_sort_knuth);
    }

    #[test]
    fn shell_sort_knuth_rand_test() {
        sort_random_test(shell_sort_knuth);
    }

}