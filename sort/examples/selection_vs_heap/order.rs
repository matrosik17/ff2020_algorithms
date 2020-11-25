extern crate gnuplot;
extern crate sort;

use gnuplot::{Figure, AxesCommon, DashType, AutoOption, PlotOption,XAxis, YAxis};

use sort::{selection_sort, heap_sort};
use sort::selection_sort::selection_sort_count_swaps;
use sort::bench_utils::*;

fn main() {
    let size = 30_000;
    let n_points = 40;
    let seed = 42;

    let cmp_ord_params = CompareOrdParams {
        group_name: "Selection Ord",
        sorts: vec![
            SortParams::new("Selecton sort", selection_sort),
            // SortParams::new("Heap sort", heap_sort),
        ],
        sample_size: 10,
    };

    let tests_collection = generate_ord_collection(size, n_points, seed);
    let cmp_ord_results = compare_time_order(&tests_collection, cmp_ord_params);

    let swaps: Vec<usize> = tests_collection.iter()
        .map(|vec| {
            let mut vec = vec.to_vec();
            selection_sort_count_swaps(&mut vec)
        })
        .collect();

    let mut fg = Figure::new();
    let axes = fg.axes2d()
        .set_title(&cmp_ord_results.group_name, &[])
        .set_x_range(AutoOption::Fix(-1.), AutoOption::Fix(1.))
        .set_x_label("Ord", &[])
        .set_y_label("Time, s", &[])
        .set_y2_label("Swaps", &[])
        .set_y2_ticks(
            Some((AutoOption::Auto, 0)),
            &[],
            &[]
        )
        .set_y2_grid(true);

    axes.lines(
        &cmp_ord_results.ord_coeffs,
        &swaps,
        &[
            PlotOption::Caption("Selection sort - swaps"),
            PlotOption::LineStyle(DashType::Dash),
            PlotOption::Axes(XAxis::X1, YAxis::Y2),
        ]
    );

    for sort_stats in cmp_ord_results.stats {
        axes
        .lines(
            &cmp_ord_results.ord_coeffs,
            &mut sort_stats.values.iter().map(|x| x.as_secs_f64()),
            &[PlotOption::Caption(sort_stats.name)]
        );
    }
    fg.show().unwrap();
}