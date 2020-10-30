use criterion::{criterion_group, criterion_main, Criterion};

extern crate sort;
use sort::selection_sort;

mod utils;
use utils::{simple_sort_bench, SimpleSortBenchParams};

fn selection_sort_benchmark(c: &mut Criterion) {
    let params = SimpleSortBenchParams {
        name: String::from("selection_sort"),
        sizes: vec![20, 100, 1000, 10_000],
    };
    simple_sort_bench(selection_sort, params, c)
}

criterion_group!(benches, selection_sort_benchmark);
criterion_main!(benches);
