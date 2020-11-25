use std::time::Duration;

use super::common::{SortParams, SortStats, measure_avg_time};

pub struct TimeSeqParams {
    pub group_name: &'static str,
    pub sorts: Vec<SortParams<i64>>,
    pub sizes: Vec<usize>,
    pub sample_size: usize,
}


#[derive(Debug)]
pub struct TimeSeqResults {
    pub group_name: &'static str,
    pub sizes: Vec<usize>,
    pub stats: Vec<SortStats<Duration>>,
}


pub fn compare_time_seq(target_vec: &[i64], mut params: TimeSeqParams) -> TimeSeqResults {
    let group_name = params.group_name;
    let sizes = params.sizes;
    let mut stats: Vec<SortStats<Duration>> = Vec::with_capacity(params.sorts.len());

    for sort_params in params.sorts.iter_mut() {

        let mut sort_stats = SortStats::with_capacity(sort_params.name, sizes.len());

        for size in sizes.iter() {
            let size = *size as usize;
            let avg_time = measure_avg_time(params.sample_size, &mut sort_params.sort, &target_vec[..size]);
            sort_stats.update(avg_time);
        }
        stats.push(sort_stats);
    }

    TimeSeqResults { group_name, sizes, stats }
}