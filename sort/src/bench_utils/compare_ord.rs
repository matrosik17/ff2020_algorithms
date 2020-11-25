use std::time::Duration;

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
    pub stats: Vec<SortStats<Duration>>,
}


pub fn compare_time_order(tests_collection: &[Vec<i64>], mut params: CompareOrdParams<i64>) -> CompareOrdResults {
    let delta_ord = 1. / params.n_points as f64;

    let group_name = params.group_name;
    let ord_coeffs: Vec<f64> = (0..tests_collection.len())
        .map(|idx| idx as f64 * delta_ord - 1.)
        .collect();
    let mut stats: Vec<SortStats<Duration>> = Vec::with_capacity(tests_collection.len());

    for sort_params in params.sorts.iter_mut() {

        let mut sort_stats = SortStats::with_capacity(sort_params.name, tests_collection.len());

        for test_vec in tests_collection.iter() {
            let avg_time = measure_avg_time(params.sample_size, &mut sort_params.sort, &test_vec);
            sort_stats.update(avg_time);
        }
        stats.push(sort_stats);
    }

    CompareOrdResults {
        group_name,
        ord_coeffs,
        stats,
    }
}