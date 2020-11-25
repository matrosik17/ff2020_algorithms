extern crate gnuplot;
extern crate sort;

use gnuplot::{Figure, AxesCommon, PlotOption};

use sort::{shell_sort, shell_sort_knuth};
use sort::bench_utils::{
    CompOrdParams,
    SortParams,
    generate_ord_collection,
    compare_comps_order,
};

fn main() {
    let size = 500;
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

    let mut fg = Figure::new();
    let axes = fg.axes2d()
        .set_title(&cmp_ord_results.group_name, &[])
        .set_x_label("Ord", &[])
        .set_y_label("Comps", &[])
        .set_y2_grid(true);

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