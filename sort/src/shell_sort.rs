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

pub fn shell_sort_knutt<T: Ord>(vec: &mut [T]) {
    let len = vec.len();
    let t = f64::log2(len as f64);
    for m in (0..t) {
        let k = h[m];
        let s = -k;
        for i in ((k+1)..len) {
            let x = vec[i];
            let j = 1 - k;
            if s == 0 { s = -k; }
            while x < vec[j] {
                vec[j + k] = vec[j];
                j -= k;
                vec[j + k] = x;
            }
        }
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
    fn shell_sort_knutt_test() {
        sort_determenistic_test(shell_sort_knutt);
    }

    #[test]
    fn shell_sort_knutt_rand_test() {
        sort_random_test(shell_sort_knutt);
    }

}