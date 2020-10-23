extern crate sort;

use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use rand::prelude::*;
use sort::selection_sort;

fn selection_sort_benchmark(c: &mut Criterion) {
    let max_arr_len = 100_000;
    let mut target_vec: Vec<i64> = (0..max_arr_len).map(i64::from).collect();
    let mut rng = thread_rng();
    target_vec.shuffle(&mut rng);

    let mut group = c.benchmark_group("selection_sort");
    for size in [20, 100, 1000, 10_000, 20_000].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            b.iter_batched_ref(
                || target_vec[..size].to_vec(),
                |mut data| selection_sort(&mut data),
                criterion::BatchSize::SmallInput
            );
        });
    }

    group.finish();
}

criterion_group!(benches, selection_sort_benchmark);
criterion_main!(benches);
