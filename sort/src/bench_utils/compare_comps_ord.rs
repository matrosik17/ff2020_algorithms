use super::common::{SortParams, SortStats, count_comps};
use super::count_wrapper::Counter;


pub struct CompOrdParams {
    pub group_name: &'static str,
    pub sorts: Vec<SortParams<Counter<i64>>>,
}


#[derive(Debug)]
pub struct CompOrdResults {
    pub group_name: &'static str,
    pub ord_coeffs: Vec<f64>,
    pub stats: Vec<SortStats<usize>>,
}


pub fn compare_comps_order(tests_collection: &[Vec<i64>], mut params: CompOrdParams) -> CompOrdResults {
    let size = tests_collection.len();
    let delta_ord = 2. / size as f64;

    let group_name = params.group_name;
    let ord_coeffs: Vec<f64> = (0..size)
        .map(|idx| idx as f64 * delta_ord - 1.)
        .collect();
    let mut stats: Vec<SortStats<usize>> = Vec::with_capacity(size);

    for sort_params in params.sorts.iter_mut() {

        let mut sort_stats = SortStats::with_capacity(sort_params.name, size);

        for test_vec in tests_collection.iter() {
            let counts = count_comps(&mut sort_params.sort, &test_vec);
            sort_stats.update(counts);
        }
        stats.push(sort_stats);
    }

    CompOrdResults {
        group_name,
        ord_coeffs,
        stats,
    }
}
