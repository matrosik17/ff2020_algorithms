extern crate gnuplot;
extern crate sort;

use gnuplot::{Figure, AxesCommon, PlotOption, Caption, AutoOption, DashType, XAxis, YAxis};

use sort::{shell_sort, shell_sort_knuth};
use sort::shell_sort::{shell_sort_count_swaps, shell_sort_knuth_count_swaps};
use sort::bench_utils::{
    CompSeqParams,
    SortParams,
    generate_seq,
    compare_comps_seq,
};


fn main() {
    let arr_size = 10_000;
    let seed = 42;

    let cmp_comp_seq_params = CompSeqParams {
        group_name: "Shell sorts",
        sorts: vec![
            SortParams::new("Normal", shell_sort),
            SortParams::new("Knuth", shell_sort_knuth),
            ],
            sizes: (5..arr_size).step_by(50).collect(),
    };
    let target_vec = generate_seq(arr_size, seed);
    let cmp_time_results = compare_comps_seq(&target_vec, cmp_comp_seq_params);

    let normal_swaps: Vec<usize> = cmp_time_results.sizes.iter()
        .map(|size| {
            let mut vec = target_vec[..*size].to_vec();
            shell_sort_count_swaps(&mut vec)
        })
        .collect();

    let knuth_swaps: Vec<usize> = cmp_time_results.sizes.iter()
        .map(|size| {
            let mut vec = target_vec[..*size].to_vec();
            shell_sort_knuth_count_swaps(&mut vec)
        })
        .collect();

    let mut fg = Figure::new();
    let axes = fg.axes2d()
        .set_title(&cmp_time_results.group_name, &[])
        .set_y_label("Comps", &[])
        .set_x_label("N", &[])
        .set_y2_label("Swaps", &[])
        .set_y2_ticks(
            Some((AutoOption::Auto, 0)),
            &[],
            &[]
        );

        axes.lines(
            &cmp_time_results.sizes,
            &normal_swaps,
            &[
                PlotOption::Caption("Normal - swaps"),
                PlotOption::LineStyle(DashType::Dash),
                PlotOption::Axes(XAxis::X1, YAxis::Y2),
            ]
        );

        axes.lines(
            &cmp_time_results.sizes,
            &knuth_swaps,
            &[
                PlotOption::Caption("Knuth - swaps"),
                PlotOption::LineStyle(DashType::Dash),
                PlotOption::Axes(XAxis::X1, YAxis::Y2),
            ]
        );

    for sort_stats in cmp_time_results.stats {
        axes.lines(
            &cmp_time_results.sizes,
            &sort_stats.values,
            &[Caption(sort_stats.name)]
        );
    }
    fg.show().unwrap();
}