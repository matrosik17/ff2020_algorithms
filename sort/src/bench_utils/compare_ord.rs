use std::time::Duration;

use super::common::{SortParams, SortStats, measure_avg_time};


pub struct TimeOrdParams {
    pub group_name: &'static str,
    pub sorts: Vec<SortParams<i64>>,
    pub sample_size: usize,
}


#[derive(Debug)]
pub struct TimeOrdResults {
    pub group_name: &'static str,
    pub ord_coeffs: Vec<f64>,
    pub stats: Vec<SortStats<Duration>>,
}


pub fn compare_time_order(tests_collection: &[Vec<i64>], mut params: TimeOrdParams) -> TimeOrdResults {
    let size = tests_collection.len();
    let delta_ord = 1. / size as f64;

    let group_name = params.group_name;
    let ord_coeffs: Vec<f64> = (0..size)
        .map(|idx| idx as f64 * delta_ord - 1.)
        .collect();
    let mut stats: Vec<SortStats<Duration>> = Vec::with_capacity(size);

    for sort_params in params.sorts.iter_mut() {

        let mut sort_stats = SortStats::with_capacity(sort_params.name, size);

        for test_vec in tests_collection.iter() {
            let avg_time = measure_avg_time(params.sample_size, &mut sort_params.sort, &test_vec);
            sort_stats.update(avg_time);
        }
        stats.push(sort_stats);
    }

    TimeOrdResults {
        group_name,
        ord_coeffs,
        stats,
    }
}