extern crate gnuplot;
extern crate sort;

use gnuplot::{Figure, Caption, AxesCommon};

use sort::{selection_sort, heap_sort};
use sort::bench_utils::*;

fn main() {
    let cmp_ord_params = CompareOrdParams {
        group_name: "Selection vs Heap",
        sorts: vec![
            SortParams::new("Selecton sort", selection_sort),
            SortParams::new("Heap sort", heap_sort),
        ],
        size: 100,
        n_points: 20,
        sample_size: 100,
        seed: 42,
    };
    let tests_collection = generate_ord_collection(
        cmp_ord_params.size,
        cmp_ord_params.n_points,
        cmp_ord_params.seed
    );
    let cmp_ord_results = compare_time_order(&tests_collection, cmp_ord_params);
    // println!("{:?}", cmp_ord_results);

    let mut fg = Figure::new();
    let axes = fg.axes2d()
        .set_title(&cmp_ord_results.group_name, &[])
        .set_y_label("Time, mcs", &[])
        .set_x_label("Ord", &[]);

    for sort_stats in cmp_ord_results.stats {
        axes.lines(
            &cmp_ord_results.ord_coeffs,
            &mut sort_stats.avg_times.iter().map(|x| x.as_nanos() as f64),
            &[Caption(sort_stats.name)]
        );
    }
    fg.show().unwrap();
    // let mut fg = Figure::new();
    // fg.axes2d()
    //     .set_title(&cmp_ord_results.group_name, &[])
    //     .set_y_label("Time, ns", &[])
    //     .set_x_label("Ord", &[])
    //     .lines(
    //         &cmp_ord_results.ord_coeffs,
    //         &avg_times1,
    //         &[Caption(cmp_ord_results.sort_name1), Color("black")]
    //     )
    //     .lines(
    //         &cmp_ord_results.ord_coeffs,
    //         &avg_times2,
    //         &[Caption(cmp_ord_results.sort_name2), Color("red")]
    //     );
    // fg.show().unwrap();
}