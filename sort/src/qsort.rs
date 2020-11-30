// TODO: доделать тесты с хоаром
// TODO: выбор опорного элемента (Сэджвик)

fn partition<T: Ord>(vec: &mut [T], first: usize, last: usize) -> usize {
    {
        let mid = (first + last) / 2;
        vec.swap(last, mid);
    };
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

fn partition_hoare<T: Ord + Copy>(vec: &mut [T], first: usize, last: usize) -> usize {
    let pivot_idx = ((first + last) as f64 / 2.).floor() as usize;
    let pivot = vec[pivot_idx];
    let mut i = first as i64 - 1;
    let mut j = last as i64 + 1;

    loop {

        loop {
            i += 1;
            if vec[i as usize] >= pivot { break; }
        }

        loop {
            j -= 1;
            if vec[j as usize] <= pivot { break; }
        }

        if i >= j { return j as usize; }
        vec.swap(i as usize, j as usize);
    }
}


fn qsort_impl<T, F>(array: &mut[T], first: usize, last: usize, partition: &mut F)
where
    T: Ord,
    F: FnMut(&mut [T], usize, usize) -> usize
{
    if first < last {
        let mid = partition(array, first, last);
        if mid != 0 { qsort_impl(array, first, mid - 1, partition); }
        qsort_impl(array, mid + 1, last, partition);
    }
}

pub fn qsort<T: Ord>(vec: &mut [T]) {
    let first = 0;
    let last = vec.len() - 1;
    qsort_impl(vec, first, last, &mut partition);
}

pub fn qsort_hoare<T: Ord + Copy>(vec: &mut [T]) {
    let first = 0;
    let last = vec.len() - 1;
    qsort_impl(vec, first, last, &mut partition_hoare);
}

pub fn qsort_iterative<T: Ord>(vec: &mut [T]) {
    let stack_size: usize = (vec.len() as f64).log2().ceil() as usize + 1;
    let mut stack = Vec::<(usize, usize)>::with_capacity(stack_size);

    let first = 0;
    let last = vec.len() - 1;
    stack.push((first, last));

    while !stack.is_empty() {
        let (first, mut last) = stack.pop().unwrap();
        while first < last {
            let mid = partition(vec, first, last);
            stack.push((mid + 1, last));

            last = match mid {
                0 => mid,
                _ => mid - 1,
            };
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::*;

    #[test]
    fn qsort_test() {
        sort_determenistic_test(qsort);
    }

    #[test]
    fn qsort_rand_test() {
        sort_random_test(qsort);
    }

    #[test]
    fn qsort_hoare_test() {
        sort_determenistic_test(qsort_hoare);
    }

    #[test]
    fn qsort_hoare_rand_test() {
        sort_random_test(qsort_hoare);
    }

    #[test]
    fn qsort_iterative_test() {
        sort_determenistic_test(qsort_iterative);
    }

    #[test]
    fn qsort_iterative_rand_test() {
        sort_random_test(qsort_iterative);
    }

}