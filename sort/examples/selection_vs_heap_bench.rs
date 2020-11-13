extern crate gnuplot;
extern crate sort;

use gnuplot::{Figure, Caption, Color, AxesCommon};

use sort::{selection_sort, heap_sort};
use sort::bench_utils::*;

fn main() {
    let arg1 = std::env::args().nth(1);
    if let Some(arg) = arg1 { println!("{}", arg); }

    // let cmp_time_params = CompareTimeParams {
    //     group_name: "Selection vs Heap",
    //     sort_name1: "Selection sort",
    //     sort_name2: "Heap sort",
    //     sizes: vec![10, 20, 50, 100, 1000],
    //     sample_size: 100,
    //     seed: 42,
    // };
    // let cmp_time_results = compare_time(selection_sort, heap_sort, cmp_time_params);
    // println!("{:?}", cmp_time_results);
    // let avg_times1: Vec<f64> = cmp_time_results.avg_times1.iter().map(|x| x.as_millis() as f64).collect();
    // let avg_times2: Vec<f64> = cmp_time_results.avg_times2.iter().map(|x| x.as_millis() as f64).collect();
    // let mut fg1 = Figure::new();
    // fg1.axes2d()
    //     .set_title(&cmp_time_results.group_name, &[])
    //     .set_y_label("Time, ms", &[])
    //     .set_x_label("N", &[])
    //     .lines(
    //         &cmp_time_results.sizes,
    //         &avg_times1,
    //         &[Caption(cmp_time_results.sort_name1), Color("black")]
    //     )
    //     .lines(
    //         &cmp_time_results.sizes,
    //         &avg_times2,
    //         &[Caption(cmp_time_results.sort_name2), Color("red")]
    //     );
    // fg1.show().unwrap();

    let cmp_ord_params = CompareOrdParams {
        group_name: "Selection vs Heap",
        sort_name1: "Selection sort",
        sort_name2: "Heap sort",
        size: 1_000,
        n_points: 20,
        sample_size: 100,
        seed: 42,
    };
    let cmp_ord_results = compare_time_order(selection_sort, heap_sort, cmp_ord_params);
    println!("{:?}", cmp_ord_results);

    let avg_times1: Vec<f64> = cmp_ord_results.avg_times1.iter().map(|x| x.as_nanos() as f64).collect();
    let avg_times2: Vec<f64> = cmp_ord_results.avg_times2.iter().map(|x| x.as_nanos() as f64).collect();
    let mut fg2 = Figure::new();
    fg2.axes2d()
        .set_title(&cmp_ord_results.group_name, &[])
        .set_y_label("Time, ns", &[])
        .set_x_label("Ord", &[])
        .lines(
            &cmp_ord_results.ord_coeffs,
            &avg_times1,
            &[Caption(cmp_ord_results.sort_name1), Color("black")]
        )
        .lines(
            &cmp_ord_results.ord_coeffs,
            &avg_times2,
            &[Caption(cmp_ord_results.sort_name2), Color("red")]
        );
    fg2.show().unwrap();
}