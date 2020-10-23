extern crate sort;

use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use rand::prelude::*;
use sort::heap_sort;

fn heap_sort_benchmark(c: &mut Criterion) {
    let max_arr_len = 10_000_000;
    let steps = vec![10, 20, 50, 100, 1000, 10_000, 20_000, 50_000, 100_000, 1_000_000, 10_000_000];
    let mut target_vec: Vec<i64> = (0..max_arr_len).map(i64::from).collect();
    let mut rng = thread_rng();
    target_vec.shuffle(&mut rng);

    let mut group = c.benchmark_group("heap_sort");
    for size in steps.iter() {
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            b.iter_batched_ref(
                || target_vec[..size].to_vec(),
                |mut data| heap_sort(&mut data),
                criterion::BatchSize::SmallInput
            );
        });
    }

    group.finish();
}

criterion_group!(benches, heap_sort_benchmark);
criterion_main!(benches);
