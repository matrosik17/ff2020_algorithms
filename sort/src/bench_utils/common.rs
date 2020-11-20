use std::time::{Duration, Instant};


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
pub struct SortStats {
    pub name: &'static str,
    pub avg_times: Vec<Duration>,
    pub n_comps: Option<Vec<usize>>,
    pub n_swaps: Option<Vec<usize>>,
}

impl SortStats {

    pub fn with_capacity(name: &'static str, size: usize) -> Self {
        Self {
            name,
            avg_times: Vec::with_capacity(size),
            n_comps: None,
            n_swaps: None
        }
    }

    pub fn update_time(&mut self, avg_time: Duration) {
        self.avg_times.push(avg_time);
    }

    pub fn update_comps(&mut self, n_comps: usize) {
        if let Some(comps) = &mut self.n_comps {
            comps.push(n_comps);
        } else {
            let mut comps = Vec::with_capacity(self.avg_times.len());
            comps.push(n_comps);
            self.n_comps = Some(comps);
        }
    }

    pub fn update_swaps(&mut self, n_swaps: usize) {
        if let Some(swaps) = &mut self.n_swaps {
            swaps.push(n_swaps);
        } else {
            let mut swaps = Vec::with_capacity(self.avg_times.len());
            swaps.push(n_swaps);
            self.n_swaps = Some(swaps);
        }
    }

}
