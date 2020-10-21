fn sift(heap: &mut [i64]) {
    let mut root: usize = 0;
    loop {
        let mut child = 2 * root + 1;
        if child >= heap.len() { break; }
        if child + 1 < heap.len() && heap[child] < heap[child + 1] {
            child += 1;
        }

        if heap[root] < heap[child] {
            heap.swap(root, child);
            root = child;
        } else {
            break;
        }
    }
    // let mut i: usize = 0;
    // let mut j: usize = 0;
    // let x = heap[0];
    // if j < heap.len() && heap[j+1] < heap[j] { j += 1; }
    // while j < heap.len() && heap[j] < x {
    //     heap.swap(i, j);
    //     i = j;
    //     j = 2 * j;
    //     if j < heap.len() && heap[j+1] < heap[j] { j += 1; }
    // }
}

pub fn heap_sort(vec: &mut [i64]) {
    let mut right_idx: usize = vec.len() - 1;
    while right_idx > 0 {
        vec.swap(0, right_idx);
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