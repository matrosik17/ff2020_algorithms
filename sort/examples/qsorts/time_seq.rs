extern crate gnuplot;
extern crate sort;

use gnuplot::{Figure, AxesCommon, PlotOption, Caption, DashType, XAxis, YAxis};

use sort::{qsort, qsort_iterative};
use sort::bench_utils::{
    TimeSeqParams,
    SortParams,
    generate_seq,
    compare_time_seq
};


fn main() {
    let arr_size = 1_000_000;
    let seed = 42;
    let target_vec = generate_seq(arr_size, seed);

    let cmp_time_params = TimeSeqParams {
        group_name: "Qsort",
        sorts: vec![
            SortParams::new("Normal", qsort),
            SortParams::new("Iterative", qsort_iterative),
        ],
        sizes: (0..arr_size).step_by(10_000).skip(1).collect(),
        sample_size: 10,
    };
    let cmp_time_results = compare_time_seq(&target_vec, cmp_time_params);
    let n_logn: Vec<f64> = cmp_time_results.sizes.iter().map(|n| {
            let n = *n as f64;
            n * n.log2()
        })
        .collect();

    let mut fg = Figure::new();
    let axes = fg.axes2d()
        .set_title(&cmp_time_results.group_name, &[])
        .set_y_label("Time, s", &[])
        .set_x_label("N", &[]);

    axes.lines(
        &cmp_time_results.sizes,
        &n_logn,
        &[
            PlotOption::Caption("N log(N)"),
            PlotOption::LineStyle(DashType::Dash),
            PlotOption::Axes(XAxis::X1, YAxis::Y2),
        ]
    );

    for sort_stats in cmp_time_results.stats {
        axes.lines(
            &cmp_time_results.sizes,
            &mut sort_stats.values.iter().map(|x| x.as_secs_f64()),
            &[Caption(sort_stats.name)]
        );
    }
    fg.show().unwrap();
}