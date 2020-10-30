use criterion::{criterion_group, criterion_main, Criterion};

extern crate sort;
use sort::heap_sort;

mod utils;
use utils::{simple_sort_bench, SimpleSortBenchParams};

fn heap_sort_benchmark(c: &mut Criterion) {
    let params = SimpleSortBenchParams {
        name: String::from("heap_sort"),
        sizes: vec![10, 1000, 10_000, 100_000, 1_000_000, 10_000_000],
    };
    simple_sort_bench(heap_sort, params, c)
}

criterion_group!(benches, heap_sort_benchmark);
criterion_main!(benches);
