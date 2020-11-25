use super::common::{SortParams, SortStats, count_comps};
use super::count_wrapper::Counter;


pub struct CompSeqParams {
    pub group_name: &'static str,
    pub sorts: Vec<SortParams<Counter<i64>>>,
    pub sizes: Vec<usize>,
    pub sample_size: usize,
}


#[derive(Debug)]
pub struct CompSeqResults {
    pub group_name: &'static str,
    pub sizes: Vec<usize>,
    pub stats: Vec<SortStats<usize>>,
}


pub fn compare_comps_seq(target_vec: &[i64], mut params: CompSeqParams) -> CompSeqResults {
    let group_name = params.group_name;
    let sizes = params.sizes;
    let mut stats: Vec<SortStats<usize>> = Vec::with_capacity(params.sorts.len());

    for sort_params in params.sorts.iter_mut() {

        let mut sort_stats = SortStats::with_capacity(sort_params.name, sizes.len());

        for size in sizes.iter() {
            let size = *size as usize;
            let counts = count_comps(&mut sort_params.sort, &target_vec[..size]);
            sort_stats.update(counts);
        }
        stats.push(sort_stats);
    }

    CompSeqResults { group_name, sizes, stats }
}