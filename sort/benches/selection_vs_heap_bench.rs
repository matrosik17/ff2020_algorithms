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

fn compare_order_sorts_benchmark(c: &mut Criterion) {

    let params = CompareOrdParams {
        group_name: String::from("heap_vs_selection/order"),
        sort_name1: String::from("heap_sort"),
        sort_name2: String::from("selection_sort"),
        size: 35_000,
        n_points: 20,
        sample_size: Some(10),
        sampling_mode: None,
    };

    compare_time_order(heap_sort, selection_sort, params, c);
}

criterion_group!(benches, compare_time_sorts_benchmark, compare_order_sorts_benchmark);
criterion_main!(benches);
