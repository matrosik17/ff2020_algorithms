use criterion::{criterion_group, criterion_main, Criterion, PlotConfiguration, AxisScale};

extern crate sort;
use sort::{qsort, qsort_iterative};

mod utils;
use utils::{compare_time, CompareTimeParams, compare_time_order, CompareOrdParams};

fn compare_time_sorts_benchmark(c: &mut Criterion) {
    let plot_config = PlotConfiguration::default()
        .summary_scale(AxisScale::Logarithmic);

    let params = CompareTimeParams {
        group_name: String::from("qsort_res_vs_it/time"),
        sort_name1: String::from("qsort"),
        sort_name2: String::from("qsort_iterative"),
        sizes: vec![10, 20, 50, 100, 1000, 10_000, 100_000, 1_000_000],
        plot_config,
        sample_size: Some(10),
        sampling_mode: None,
    };

    compare_time(qsort, qsort_iterative, params, c);
}

fn compare_order_sorts_1k_benchmark(c: &mut Criterion) {

    let params = CompareOrdParams {
        group_name: String::from("qsort_res_vs_it_1k_order"),
        sort_name1: String::from("qsort"),
        sort_name2: String::from("qsort_iterative"),
        size: 1_000,
        n_points: 20,
        sample_size: Some(100),
        sampling_mode: None,
    };

    compare_time_order(qsort, qsort_iterative, params, c);
}

fn compare_order_sorts_10k_benchmark(c: &mut Criterion) {

    let params = CompareOrdParams {
        group_name: String::from("qsort_res_vs_it_10k_order"),
        sort_name1: String::from("qsort"),
        sort_name2: String::from("qsort_iterative"),
        size: 10_000,
        n_points: 20,
        sample_size: Some(100),
        sampling_mode: None,
    };

    compare_time_order(qsort, qsort_iterative, params, c);
}

fn compare_order_sorts_35k_benchmark(c: &mut Criterion) {

    let params = CompareOrdParams {
        group_name: String::from("qsort_res_vs_it_35k_order"),
        sort_name1: String::from("qsort"),
        sort_name2: String::from("qsort_iterative"),
        size: 35_000,
        n_points: 20,
        sample_size: Some(10),
        sampling_mode: None,
    };

    compare_time_order(qsort, qsort_iterative, params, c);
}

criterion_group!(benches,
    compare_time_sorts_benchmark,
    compare_order_sorts_1k_benchmark,
    compare_order_sorts_10k_benchmark,
    compare_order_sorts_35k_benchmark
);
criterion_main!(benches);
