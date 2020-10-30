use criterion::{Criterion, BenchmarkId, PlotConfiguration};
use rand::prelude::*;

pub struct SimpleSortBenchParams {
    pub name: String,
    pub sizes: Vec<usize>,
}

pub fn simple_sort_bench<F: Fn(&mut [i64])>(sort: F, params: SimpleSortBenchParams, c: &mut Criterion) {
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

pub struct CompareTimeParams {
    pub group_name: String,
    pub sort_name1: String,
    pub sort_name2: String,
    pub sizes: Vec<usize>,
    pub plot_config: PlotConfiguration,
}

pub fn compare_time<F1: Fn(&mut [i64]), F2: Fn(&mut [i64])>(
    sort1: F1,
    sort2: F2,
    params: CompareTimeParams,
    c: &mut Criterion
) {
    let max_arr_len = *params.sizes.last().unwrap() as i64;
    let mut target_vec: Vec<i64> = (0..max_arr_len).map(i64::from).collect();
    let mut rng = thread_rng();
    target_vec.shuffle(&mut rng);

    let mut group = c.benchmark_group(params.group_name);
    group.plot_config(params.plot_config);

    for size in params.sizes.iter() {

        group.bench_with_input(BenchmarkId::new(&params.sort_name1, size), size, |b, &size| {
            b.iter_batched_ref(
                || target_vec[..size].to_vec(),
                |mut data| sort1(&mut data),
                criterion::BatchSize::SmallInput
            );
        });

        group.bench_with_input(BenchmarkId::new(&params.sort_name2, size), size, |b, &size| {
            b.iter_batched_ref(
                || target_vec[..size].to_vec(),
                |mut data| sort2(&mut data),
                criterion::BatchSize::SmallInput
            );
        });

    }

    group.finish();
}

use std::time::Duration;
// use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId, PlotConfiguration, AxisScale};
// use rand::prelude::*;
// use sort::{heap_sort, selection_sort};

pub struct CompareOrdParams {
    pub group_name: String,
    pub sort_name1: String,
    pub sort_name2: String,
    pub size: i64,
    pub n_points: i64,
}

fn bubble_sort_iter<T: Ord>(arr: &mut [T]) {
    for i in 0..arr.len()-1 {
        if arr[i] > arr[i+1] {
            arr.swap(i, i+1);
        }
    }
}

pub fn compare_time_order<F1: Fn(&mut [i64]), F2: Fn(&mut [i64])>(
    sort1: F1,
    sort2: F2,
    params: CompareOrdParams,
    c: &mut Criterion
) {

    let step_size = (params.size / params.n_points) as usize;
    let mut target_vec: Vec<i64> = (0..params.size).map(i64::from).map(|x| x - params.size/2).rev().collect();

    let mut group = c.benchmark_group(params.group_name);
    group.warm_up_time(Duration::from_millis(50));
    group.measurement_time(Duration::from_millis(150));

    for step in (0..=params.size).step_by(step_size) {
        let ord_coeff = step as f64 / params.size as f64;
        for _ in 0..step_size {
            bubble_sort_iter(&mut target_vec);
        }

        group.bench_with_input(BenchmarkId::new(params.sort_name1.clone(), ord_coeff), &ord_coeff, |b, &_ord_coeff| {
            b.iter_batched_ref(
                || target_vec.clone(),
                |mut data| sort1(&mut data),
                criterion::BatchSize::SmallInput
            );
        });

        group.bench_with_input(BenchmarkId::new(params.sort_name2.clone(), ord_coeff), &ord_coeff, |b, &_ord_coeff| {
            b.iter_batched_ref(
                || target_vec.clone(),
                |mut data| sort2(&mut data),
                criterion::BatchSize::SmallInput
            );
        });

    }

    group.finish();
}
