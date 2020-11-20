use rand::prelude::*;
use rand::rngs::StdRng;

use super::common::{SortParams, SortStats, measure_avg_time};


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


pub fn generate_ord_collection(arr_size: usize, n_ord_points: usize, seed: u64) -> Vec<Vec<i64>> {
    let step_size = (arr_size / n_ord_points) as usize;
    let mut rand_vec: Vec<i64> = (0..(arr_size as i64)).collect();
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

pub fn compare_time_order(tests_collection: &[Vec<i64>], mut params: CompareOrdParams<i64>) -> CompareOrdResults {
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