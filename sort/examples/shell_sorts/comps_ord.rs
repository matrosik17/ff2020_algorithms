extern crate gnuplot;
extern crate sort;

use gnuplot::{Figure, AxesCommon, PlotOption, AutoOption, DashType, XAxis, YAxis};

use sort::{shell_sort, shell_sort_knuth};
use sort::shell_sort::{shell_sort_count_swaps, shell_sort_knuth_count_swaps};
use sort::bench_utils::{
    CompOrdParams,
    SortParams,
    generate_ord_collection,
    compare_comps_order,
};

fn main() {
    let size = 5_000;
    let n_points = 40;
    let seed = 42;

    let cmp_comp_ord_params = CompOrdParams {
        group_name: "Shell sorts",
        sorts: vec![
            SortParams::new("Normal", shell_sort),
            SortParams::new("Knuth", shell_sort_knuth),
        ],
    };

    let tests_collection = generate_ord_collection(size, n_points, seed);
    let cmp_ord_results = compare_comps_order(&tests_collection, cmp_comp_ord_params);

    let normal_swaps: Vec<usize> = tests_collection.iter()
        .map(|vec| {
            let mut vec = vec.to_vec();
            shell_sort_count_swaps(&mut vec)
        })
        .collect();

    let knuth_swaps: Vec<usize> = tests_collection.iter()
        .map(|vec| {
            let mut vec = vec.to_vec();
            shell_sort_knuth_count_swaps(&mut vec)
        })
        .collect();

    let mut fg = Figure::new();
    let axes = fg.axes2d()
        .set_title(&cmp_ord_results.group_name, &[])
        .set_x_label("Ord", &[])
        .set_y_label("Comps", &[])
        .set_y2_label("Swaps", &[])
        .set_y2_ticks(
            Some((AutoOption::Auto, 0)),
            &[],
            &[]
        )
        .set_y2_grid(true);

    axes.lines(
        &cmp_ord_results.ord_coeffs,
        &normal_swaps,
        &[
            PlotOption::Caption("Normal - swaps"),
            PlotOption::LineStyle(DashType::Dash),
            PlotOption::Axes(XAxis::X1, YAxis::Y2),
        ]
    );

    axes.lines(
        &cmp_ord_results.ord_coeffs,
        &knuth_swaps,
        &[
            PlotOption::Caption("Knuth - swaps"),
            PlotOption::LineStyle(DashType::Dash),
            PlotOption::Axes(XAxis::X1, YAxis::Y2),
        ]
    );

    for sort_stats in cmp_ord_results.stats {
        axes
        .lines(
            &cmp_ord_results.ord_coeffs,
            &sort_stats.values,
            &[PlotOption::Caption(sort_stats.name)]
        );
    }
    fg.show().unwrap();
}