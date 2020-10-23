extern crate sort;

use std::time::Duration;
use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId, PlotConfiguration, AxisScale};
use rand::prelude::*;
use sort::{heap_sort, selection_sort};

fn compare_time_sorts_benchmark(c: &mut Criterion) {
    let plot_config = PlotConfiguration::default()
        .summary_scale(AxisScale::Logarithmic);

    let steps = vec![10, 20, 50, 100, 1000, 10_000, 20_000];
    let mut target_vec: Vec<i64> = (0..=20_000).map(i64::from).collect();
    let mut rng = thread_rng();
    target_vec.shuffle(&mut rng);

    let mut group = c.benchmark_group("heap_vs_selection/time");
    group.plot_config(plot_config);

    for size in steps.iter() {

        group.bench_with_input(BenchmarkId::new("heap_sort", size), size, |b, &size| {
            b.iter_batched_ref(
                || target_vec[..size].to_vec(),
                |mut data| heap_sort(&mut data),
                criterion::BatchSize::SmallInput
            );
        });

        group.bench_with_input(BenchmarkId::new("selection_sort", size), size, |b, &size| {
            b.iter_batched_ref(
                || target_vec[..size].to_vec(),
                |mut data| selection_sort(&mut data),
                criterion::BatchSize::SmallInput
            );
        });

    }

    group.finish();
}

fn bubble_sort_iter<T: Ord>(arr: &mut [T]) {
    for i in 0..arr.len()-1 {
        if arr[i] > arr[i+1] {
            arr.swap(i, i+1);
        }
    }
}

fn compare_order_sorts_benchmark(c: &mut Criterion) {

    let arr_size = 100;
    let n_points = 10;
    let step_size = (arr_size / n_points) as usize;
    let mut target_vec: Vec<i64> = (0..arr_size).map(i64::from).map(|x| x - arr_size/2).rev().collect();

    let mut group = c.benchmark_group("heap_vs_selection/order");
    group.warm_up_time(Duration::from_millis(50));
    group.measurement_time(Duration::from_millis(150));

    for step in (0..=arr_size).step_by(step_size) {
        let ord_coeff = step as f64 / arr_size as f64;
        for _ in 0..step_size {
            bubble_sort_iter(&mut target_vec);
        }

        group.bench_with_input(BenchmarkId::new("heap_sort", ord_coeff), &ord_coeff, |b, &_ord_coeff| {
            b.iter_batched_ref(
                || target_vec.clone(),
                |mut data| heap_sort(&mut data),
                criterion::BatchSize::SmallInput
            );
        });

        group.bench_with_input(BenchmarkId::new("selection_sort", ord_coeff), &ord_coeff, |b, &_ord_coeff| {
            b.iter_batched_ref(
                || target_vec.clone(),
                |mut data| selection_sort(&mut data),
                criterion::BatchSize::SmallInput
            );
        });

    }

    group.finish();
}

criterion_group!(benches, compare_time_sorts_benchmark, compare_order_sorts_benchmark);
criterion_main!(benches);
