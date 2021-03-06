use criterion::{criterion_group, criterion_main, Criterion, PlotConfiguration, AxisScale, SamplingMode};

extern crate sort;
use sort::{heap_sort, selection_sort};

mod utils;
use utils::{compare_time, CompareTimeParams, compare_time_order, CompareOrdParams};

fn compare_time_sorts_benchmark(c: &mut Criterion) {
    let plot_config = PlotConfiguration::default()
        .summary_scale(AxisScale::Logarithmic);

    let params = CompareTimeParams {
        group_name: String::from("heap_vs_selection/time"),
        sort_name1: String::from("heap_sort"),
        sort_name2: String::from("selection_sort"),
        sizes: vec![10, 20, 50, 100, 1000, 10_000, 100_000],
        plot_config,
        sample_size: Some(10),
        sampling_mode: Some(SamplingMode::Flat),
    };

    compare_time(heap_sort, selection_sort, params, c);
}

fn compare_order_sorts_20_benchmark(c: &mut Criterion) {

    let params = CompareOrdParams {
        group_name: String::from("heap_vs_selection_20_order"),
        sort_name1: String::from("heap_sort"),
        sort_name2: String::from("selection_sort"),
        size: 20,
        n_points: 20,
        sample_size: Some(100),
        sampling_mode: None,
    };

    compare_time_order(heap_sort, selection_sort, params, c);
}

fn compare_order_sorts_100_benchmark(c: &mut Criterion) {

    let params = CompareOrdParams {
        group_name: String::from("heap_vs_selection_100_order"),
        sort_name1: String::from("heap_sort"),
        sort_name2: String::from("selection_sort"),
        size: 100,
        n_points: 20,
        sample_size: Some(100),
        sampling_mode: None,
    };

    compare_time_order(heap_sort, selection_sort, params, c);
}

fn compare_order_sorts_1k_benchmark(c: &mut Criterion) {

    let params = CompareOrdParams {
        group_name: String::from("heap_vs_selection_1k_order"),
        sort_name1: String::from("heap_sort"),
        sort_name2: String::from("selection_sort"),
        size: 1_000,
        n_points: 20,
        sample_size: Some(100),
        sampling_mode: None,
    };

    compare_time_order(heap_sort, selection_sort, params, c);
}

fn compare_order_sorts_10k_benchmark(c: &mut Criterion) {

    let params = CompareOrdParams {
        group_name: String::from("heap_vs_selection_10k_order"),
        sort_name1: String::from("heap_sort"),
        sort_name2: String::from("selection_sort"),
        size: 10_000,
        n_points: 20,
        sample_size: Some(10),
        sampling_mode: None,
    };

    compare_time_order(heap_sort, selection_sort, params, c);
}

fn compare_order_sorts_35k_benchmark(c: &mut Criterion) {

    let params = CompareOrdParams {
        group_name: String::from("heap_vs_selection_35k_order"),
        sort_name1: String::from("heap_sort"),
        sort_name2: String::from("selection_sort"),
        size: 35_000,
        n_points: 20,
        sample_size: Some(10),
        sampling_mode: None,
    };

    compare_time_order(heap_sort, selection_sort, params, c);
}

criterion_group!(benches,
    compare_time_sorts_benchmark,
    compare_order_sorts_20_benchmark,
    compare_order_sorts_100_benchmark,
    compare_order_sorts_1k_benchmark,
    compare_order_sorts_10k_benchmark,
    compare_order_sorts_35k_benchmark
);
criterion_main!(benches);
