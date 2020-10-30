use criterion::{Criterion, BenchmarkId};
use rand::prelude::*;

pub struct SimpleSortBenchParams {
    pub name: String,
    pub sizes: Vec<usize>,
    pub random_state: Option<usize>,
}

pub fn simple_sort_bench<F: Fn(&mut [i64])>(sort: F, params: SortBenchParams, c: &mut Criterion) {
    let max_arr_len = *params.sizes.last().unwrap() as i64;
    let mut target_vec: Vec<i64> = (0..max_arr_len).map(i64::from).collect();
    let mut rng = thread_rng();
    target_vec.shuffle(&mut rng);

    let mut group = c.benchmark_group(params.name);
    for size in params.sizes.iter() {
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            b.iter_batched_ref(
                || target_vec[..size].to_vec(),
                |mut data| sort(&mut data),
                criterion::BatchSize::SmallInput
            );
        });
    }

    group.finish();
}
