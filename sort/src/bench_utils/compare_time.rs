use rand::prelude::*;
use rand::rngs::StdRng;

use super::common::{SortParams, SortStats, measure_avg_time};

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