use std::time::{Duration, Instant};

use rand::prelude::*;
use rand::rngs::StdRng;


fn measure_time<T: Ord, F: FnMut(&mut [T])>(mut sort: F, vec: &mut [T]) -> Duration {
    let start = Instant::now();
    sort(vec);
    start.elapsed()
}


fn measure_avg_time<T, F>(sample_size: usize, mut sort: F, vec: &[T]) -> Duration
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


pub struct CompareTimeParams<T: Ord> {
    pub group_name: &'static str,
    pub sorts: Vec<SortParams<T>>,
    pub sizes: Vec<usize>,
    pub sample_size: usize,
    pub seed: u64,
}


#[derive(Debug)]
pub struct CompareTimeResults {
    pub group_name: &'static str,
    pub sizes: Vec<usize>,
    pub stats: Vec<SortStats>,
}


pub fn compare_time(mut params: CompareTimeParams<i64>) -> CompareTimeResults {
    let max_arr_len = *params.sizes.last().unwrap() as i64;
    let mut target_vec: Vec<i64> = (0..max_arr_len).map(i64::from).collect();
    let mut rng = StdRng::seed_from_u64(params.seed);
    target_vec.shuffle(&mut rng);

    let group_name = params.group_name;
    let sizes = params.sizes;
    let mut stats: Vec<SortStats> = Vec::with_capacity(params.sorts.len());

    for sort_params in params.sorts.iter_mut() {

        let mut sort_stats = SortStats::with_capacity(sort_params.name, sizes.len());

        for size in sizes.iter() {
            let size = *size as usize;
            let avg_time = measure_avg_time(params.sample_size, &mut sort_params.sort, &target_vec[..size]);
            sort_stats.update_time(avg_time);
        }
        stats.push(sort_stats);
    }

    CompareTimeResults { group_name, sizes, stats }
}



pub struct CompareOrdParams<T: Ord> {
    pub group_name: &'static str,
    pub sorts: Vec<SortParams<T>>,
    pub size: usize,
    pub n_points: usize,
    pub sample_size: usize,
    pub seed: u64,
}


#[derive(Debug)]
pub struct CompareOrdResults {
    pub group_name: &'static str,
    pub ord_coeffs: Vec<f64>,
    pub stats: Vec<SortStats>,
}


fn bubble_sort_iter<T: Ord>(arr: &mut [T]) {
    for i in 0..arr.len()-1 {
        if arr[i] > arr[i+1] {
            arr.swap(i, i+1);
        }
    }
}


fn contra_bubble_sort_iter<T: Ord>(arr: &mut [T]) {
    for i in 0..arr.len()-1 {
        if arr[i] < arr[i+1] {
            arr.swap(i, i+1);
        }
    }
}


fn generate_ord_collection(arr_size: usize, n_ord_points: usize, seed: u64) -> Vec<Vec<i64>> {
    let step_size = (arr_size / n_ord_points) as usize;
    let mut rand_vec: Vec<i64> = (0..(arr_size as i64)).collect();
    // let mut rng = thread_rng();
    let mut rng = StdRng::seed_from_u64(seed);
    rand_vec.shuffle(&mut rng);

    // ordered collection
    let mut ordered_collection = Vec::with_capacity(n_ord_points + 1);
    let mut ord_vec = rand_vec.clone();
    ordered_collection.push(ord_vec.clone());
    for _ in 0..=n_ord_points {
        for _ in 0..step_size {
            bubble_sort_iter(&mut ord_vec);
        }
        ordered_collection.push(ord_vec.clone());
    }

    // contra ordered collection
    let mut contra_ordered_collection = Vec::with_capacity(n_ord_points);
    let mut contra_ord_vec = rand_vec.clone();
    for _ in 0..n_ord_points {
        for _ in 0..step_size {
           contra_bubble_sort_iter(&mut contra_ord_vec);
        }
        contra_ordered_collection.push(contra_ord_vec.clone());
    }

    // concat results
    let contra_ord_iter = contra_ordered_collection.into_iter().rev();
    let ord_iter = ordered_collection.into_iter();
    let collection: Vec<Vec<i64>> = contra_ord_iter.chain(ord_iter).collect();
    collection
}

pub fn compare_time_order(mut params: CompareOrdParams<i64>) -> CompareOrdResults {
    let tests_collection = generate_ord_collection(params.size, params.n_points, params.seed);
    let delta_ord = 1. / params.n_points as f64;

    let group_name = params.group_name;
    let ord_coeffs: Vec<f64> = (0..tests_collection.len())
        .map(|idx| idx as f64 * delta_ord - 1.)
        .collect();
    let mut stats: Vec<SortStats> = Vec::with_capacity(tests_collection.len());

    for sort_params in params.sorts.iter_mut() {

        let mut sort_stats = SortStats::with_capacity(sort_params.name, tests_collection.len());

        for test_vec in tests_collection.iter() {
            let avg_time = measure_avg_time(params.size, &mut sort_params.sort, &test_vec);
            sort_stats.update_time(avg_time);
        }
        stats.push(sort_stats);
    }

    CompareOrdResults {
        group_name,
        ord_coeffs,
        stats,
    }
}
