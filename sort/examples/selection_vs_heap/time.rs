extern crate gnuplot;
extern crate sort;

use gnuplot::{Figure, Caption, AxesCommon};

use sort::{selection_sort, heap_sort};
use sort::bench_utils::{
    TimeSeqParams,
    SortParams,
    generate_seq,
    compare_time_seq
};


fn main() {
    let arr_size = 1_000;
    let seed = 42;
    let target_vec = generate_seq(arr_size, seed);

    let cmp_time_params = TimeSeqParams {
        group_name: "Selection vs Heap",
        sorts: vec![
            SortParams::new("Selecton sort", selection_sort),
            SortParams::new("Heap sort", heap_sort),
        ],
        sizes: (5..arr_size).step_by(50).collect(),
        sample_size: 200,
    };
    let cmp_time_results = compare_time_seq(&target_vec, cmp_time_params);

    let mut fg = Figure::new();
    let axes = fg.axes2d()
        .set_title(&cmp_time_results.group_name, &[])
        .set_y_label("Time, mcs", &[])
        .set_x_label("N", &[]);

    for sort_stats in cmp_time_results.stats {
        axes.lines(
            &cmp_time_results.sizes,
            &mut sort_stats.values.iter().map(|x| x.as_secs_f64()),
            &[Caption(sort_stats.name)]
        );
    }
    fg.show().unwrap();
}