use std::time::{Duration, Instant};

use super::count_wrapper::Counter;


fn measure_time<T: Ord, F: FnMut(&mut [T])>(mut sort: F, vec: &mut [T]) -> Duration {
    let start = Instant::now();
    sort(vec);
    start.elapsed()
}


pub fn measure_avg_time<T, F>(sample_size: usize, mut sort: F, vec: &[T]) -> Duration
where
    T: Ord + Clone,
    F: FnMut(&mut [T])
{
    let mut total_duration = Duration::from_nanos(0);
    for _ in 0..sample_size {
        let mut test_vec = vec.to_vec();
        total_duration += measure_time(&mut sort, &mut test_vec);
    }
    total_duration.div_f64(sample_size as f64)
}


pub fn count_comps<T, F>(mut sort: F, vec: &[T]) -> usize
where
    T: Ord + Clone,
    F: FnMut(&mut [Counter<T>])
{
    let mut counted_vec: Vec<Counter<T>> = vec.iter()
        .map(|x| Counter::new(x.clone()))
        .collect();
    sort(&mut counted_vec);
    counted_vec.iter().map(|x| x.get_comps()).sum::<usize>() / 2
}


pub struct SortParams<T: Ord> {
    pub name: &'static str,
    pub sort: fn(&mut [T]),
}

impl<T: Ord> SortParams<T> {
    pub fn new(name: &'static str, sort: fn(&mut [T])) -> Self {
        Self { name, sort }
    }
}


#[derive(Debug)]
pub struct SortStats<T> {
    pub name: &'static str,
    pub values: Vec<T>,
}

impl<T> SortStats<T> {

    pub fn new(name: &'static str) -> Self {
        Self {
            name,
            values: Vec::new(),
        }
    }

    pub fn with_capacity(name: &'static str, size: usize) -> Self {
        Self {
            name,
            values: Vec::with_capacity(size),
        }
    }

    pub fn update(&mut self, value: T) {
        self.values.push(value);
    }

}
