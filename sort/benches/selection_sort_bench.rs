use criterion::{criterion_group, criterion_main, Criterion, SamplingMode};

extern crate sort;
use sort::selection_sort;

mod utils;
use utils::{simple_sort_bench, SimpleSortBenchParams};

fn selection_sort_benchmark(c: &mut Criterion) {
    let params = SimpleSortBenchParams {
        name: String::from("selection_sort"),
        sizes: vec![20, 100, 200, 300, 500, 1_000, 2_000, 3_000, 5_000, 7_000, 10_000, 20_000, 30_000, 50_000],
        sample_size: Some(10),
        sampling_mode: None,
    };
    simple_sort_bench(selection_sort, params, c)
}

criterion_group!(benches, selection_sort_benchmark);
criterion_main!(benches);
