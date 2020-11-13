use std::time::{Duration, Instant};

use rand::prelude::*;
use rand::rngs::StdRng;


fn measure_time<T: Ord, F: Fn(&mut [T])>(sort: F, vec: &mut [T]) -> Duration {
    let start = Instant::now();
    sort(vec);
    start.elapsed()
}

fn measure_avg_time<T, F>(sample_size: usize, sort: F, vec: &[T]) -> Duration
where
    T: Ord + Clone,
    F: Fn(&mut [T])
{
    let mut total_duration = Duration::from_nanos(0);
    for _ in 0..sample_size {
        let mut test_vec = vec.to_vec();
        total_duration += measure_time(&sort, &mut test_vec);
    }
    total_duration.div_f64(sample_size as f64)
}


pub struct CompareTimeParams {
    pub group_name: &'static str,
    pub sort_name1: &'static str,
    pub sort_name2: &'static str,
    pub sizes: Vec<usize>,
    pub sample_size: usize,
    seed: u64,
}

#[derive(Debug)]
pub struct CompareTimeResults {
    pub group_name: &'static str,
    pub sort_name1: &'static str,
    pub sort_name2: &'static str,
    pub sizes: Vec<usize>,
    pub avg_times1: Vec<Duration>,
    pub avg_times2: Vec<Duration>,
}

pub fn compare_time<F1, F2>(sort1: F1, sort2: F2,  params: CompareTimeParams) -> CompareTimeResults
where
    F1: Fn(&mut [i64]),
    F2: Fn(&mut [i64])
{
    let max_arr_len = *params.sizes.last().unwrap() as i64;
    let mut target_vec: Vec<i64> = (0..max_arr_len).map(i64::from).collect();
    // let mut rng = thread_rng();
    let mut rng = StdRng::seed_from_u64(params.seed);
    target_vec.shuffle(&mut rng);

    let mut avg_times1 = Vec::with_capacity(params.sizes.len());
    let mut avg_times2 = Vec::with_capacity(params.sizes.len());

    for size in params.sizes.iter() {
        let size = *size as usize;

        let avg_time1 = measure_avg_time(params.sample_size, &sort1, &target_vec[..size]);
        avg_times1.push(avg_time1);

        let avg_time2 = measure_avg_time(params.sample_size, &sort2, &target_vec[..size]);
        avg_times2.push(avg_time2);

    }

    CompareTimeResults {
        group_name: params.group_name,
        sort_name1: params.sort_name1,
        sort_name2: params.sort_name2,
        sizes: params.sizes,
        avg_times1,
        avg_times2,
    }
}


pub struct CompareOrdParams {
    pub group_name: &'static str,
    pub sort_name1: &'static str,
    pub sort_name2: &'static str,
    pub size: usize,
    pub n_points: usize,
    pub sample_size: usize,
    pub seed: u64,
}

#[derive(Debug)]
pub struct CompareOrdResults {
    pub group_name: &'static str,
    pub sort_name1: &'static str,
    pub sort_name2: &'static str,
    pub ord_coeffs: Vec<f64>,
    pub avg_times1: Vec<Duration>,
    pub avg_times2: Vec<Duration>,
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

pub fn compare_time_order<F1, F2>(sort1: F1, sort2: F2, params: CompareOrdParams) -> CompareOrdResults
where
    F1: Fn(&mut [i64]),
    F2: Fn(&mut [i64]),
{
    let tests_collection = generate_ord_collection(params.size, params.n_points, params.seed);
    let delta_ord = 1. / params.n_points as f64;
    let collection_size = tests_collection.len();

    let mut ord_coeffs = Vec::with_capacity(collection_size);
    let mut avg_times1 = Vec::with_capacity(collection_size);
    let mut avg_times2 = Vec::with_capacity(collection_size);

    let mut ord = -1.;
    for test_vec in tests_collection.into_iter() {
        let avg_time1 = measure_avg_time(params.sample_size, &sort1, &test_vec);
        let avg_time2 = measure_avg_time(params.sample_size, &sort2, &test_vec);
        ord_coeffs.push(ord);
        avg_times1.push(avg_time1);
        avg_times2.push(avg_time2);
        ord += delta_ord;
    }

    CompareOrdResults {
        group_name: params.group_name,
        sort_name1: params.sort_name1,
        sort_name2: params.sort_name2,
        ord_coeffs,
        avg_times1,
        avg_times2,
    }
}

pub fn compare_time_order_random<F1, F2>(sort1: F1, sort2: F2, params: CompareOrdParams) -> CompareOrdResults
where
    F1: Fn(&mut [i64]),
    F2: Fn(&mut [i64]),
{
    let step_size = (params.size / params.n_points) as usize;
    let mut target_vec: Vec<i64> = (0..(params.size as i64)).map(i64::from).collect();
    let mut rng = thread_rng();
    target_vec.shuffle(&mut rng);

    let mut ord_coeffs = Vec::with_capacity(2 * params.n_points - 1);
    let mut avg_times1 = Vec::with_capacity(2 * params.n_points - 1);
    let mut avg_times2 = Vec::with_capacity(2 * params.n_points - 1);

    // unordered arrays
    let ord = 0.;
    let avg_time1 = measure_avg_time(params.sample_size, &sort1, &target_vec);
    let avg_time2 = measure_avg_time(params.sample_size, &sort2, &target_vec);

    ord_coeffs.push(ord);
    avg_times1.push(avg_time1);
    avg_times2.push(avg_time2);

    // ordered arrays
    let mut ord_vec = target_vec.clone();
    for step in (step_size..=(params.size as usize)).step_by(step_size) {
        let ord = step as f64 / params.size as f64;
        for _ in 0..step_size {
            bubble_sort_iter(&mut ord_vec);
        }

        let avg_time1 = measure_avg_time(params.sample_size, &sort1, &ord_vec);
        let avg_time2 = measure_avg_time(params.sample_size, &sort2, &ord_vec);

        ord_coeffs.push(ord);
        avg_times1.push(avg_time1);
        avg_times2.push(avg_time2);
    }

    // contra ordered
    for step in (step_size..=(params.size as usize)).step_by(step_size) {
        let ord = - (step as f64 / params.size as f64);
        for _ in 0..step_size {
            contra_bubble_sort_iter(&mut target_vec);
        }

        let avg_time1 = measure_avg_time(params.sample_size, &sort1, &target_vec);
        let avg_time2 = measure_avg_time(params.sample_size, &sort2, &target_vec);

        ord_coeffs.push(ord);
        avg_times1.push(avg_time1);
        avg_times2.push(avg_time2);
    }

    CompareOrdResults {
        group_name: params.group_name,
        sort_name1: params.sort_name1,
        sort_name2: params.sort_name2,
        ord_coeffs,
        avg_times1,
        avg_times2,
    }
}