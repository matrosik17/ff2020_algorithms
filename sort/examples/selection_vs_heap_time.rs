extern crate gnuplot;
extern crate sort;

use gnuplot::{Figure, Caption, AxesCommon};

use sort::{selection_sort, heap_sort};
use sort::bench_utils::{CompareTimeParams, SortParams, compare_time};


fn main() {
    let cmp_time_params = CompareTimeParams {
        group_name: "Selection vs Heap",
        sorts: vec![
            SortParams::new("Selecton sort", selection_sort),
            SortParams::new("Heap sort", heap_sort),
        ],
        sizes: (5..1_000).step_by(50).collect(),
        sample_size: 200,
        seed: 42,
    };
    let cmp_time_results = compare_time(cmp_time_params);

    let mut fg = Figure::new();
    let axes = fg.axes2d()
        .set_title(&cmp_time_results.group_name, &[])
        .set_y_label("Time, mcs", &[])
        .set_x_label("N", &[]);

    for sort_stats in cmp_time_results.stats {
        axes.lines(
            &cmp_time_results.sizes,
            &mut sort_stats.avg_times.iter().map(|x| x.as_micros() as f64),
            &[Caption(sort_stats.name)]
        );
    }
    fg.show().unwrap();
}