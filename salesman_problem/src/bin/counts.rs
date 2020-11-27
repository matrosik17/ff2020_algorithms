use gnuplot::{Figure, PlotOption, AxesCommon};

use salesman_problem::*;

fn main() {
    let size = 15;
    let points = generate_points(size, 42);

    let normal_tsp_max_size = 12;
    let optimal_tsp_max_size = size;

    let mut normal_counts = Vec::with_capacity(normal_tsp_max_size);
    let mut optimal_counts = Vec::with_capacity(optimal_tsp_max_size);

    for len in 1..=size {
        if len <= normal_tsp_max_size {
            let (_, count) = find_optimal_path(&points[..len]);
            normal_counts.push(count);
        }
        if len <= optimal_tsp_max_size {
            let (_, count) = find_optimal_path_optimal(&points[..len]);
            optimal_counts.push(count);
        }
    }


    let mut fg = Figure::new();
    fg.axes2d()
        .set_title("Salesman problem - complexity", &[])
        .set_x_label("N", &[])
        .set_y_label("Num paths", &[])
        .set_y_log(Some(2.))
        .lines(
            1..=normal_tsp_max_size,
            normal_counts,
            &[PlotOption::Caption("Normal TSP")]
        )
        .lines(
            1..=optimal_tsp_max_size,
            optimal_counts,
            &[PlotOption::Caption("Optimized TSP")]
        );
    fg.show().unwrap();
    fg.close();
}
