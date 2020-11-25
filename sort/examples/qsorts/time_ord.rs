extern crate gnuplot;
extern crate sort;

use gnuplot::{Figure, AxesCommon, PlotOption};

use sort::{qsort, qsort_iterative};
use sort::bench_utils::*;

fn main() {
    let size = 1_000;
    let n_points = 40;
    let seed = 42;

    let cmp_ord_params = TimeOrdParams {
        group_name: "Qsort",
        sorts: vec![
            SortParams::new("Normal", qsort),
            SortParams::new("Iterative", qsort_iterative),
        ],
        sample_size: 10,
    };

    let tests_collection = generate_ord_collection(size, n_points, seed);
    let cmp_ord_results = compare_time_order(&tests_collection, cmp_ord_params);

    let mut fg = Figure::new();
    let axes = fg.axes2d()
        .set_title(&cmp_ord_results.group_name, &[])
        .set_x_label("Ord", &[])
        .set_y_label("Time, s", &[]);

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