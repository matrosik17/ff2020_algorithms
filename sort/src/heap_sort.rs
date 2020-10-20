fn sift(heap: &mut [i64]) {
    let mut i: usize = 0;
    let mut j: usize = 0;
    let x = heap[0];
    if j < heap.len() && heap[j+1] < heap[j] { j += 1; }
    while j < heap.len() && heap[j] < x {
        heap[i] = heap[j];
        i = j;
        j = 2 * j;
        if j < heap.len() && heap[j+1] < heap[j] { j += 1; }
    }

}

pub fn heap_sort(vec: &mut [i64]) {
    let mut right_idx: usize = vec.len();
    while right_idx > 0 {
        let x = vec[0];
        vec[0] = vec[right_idx];
        vec[right_idx] = x;
        right_idx -= 1;
        sift(&mut vec[..right_idx]);
    }
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