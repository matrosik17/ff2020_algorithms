use gnuplot::{Figure, PlotOption, Coordinate, ArrowheadType, AxesCommon};

use salesman_problem::*;

fn main() {
    let size = 14;
    let points = generate_points(size, 42);
    let (path, count) = find_optimal_path_optimal(&points);
    println!("count: {}", count);

    let mut fg = Figure::new();
    let axes = fg.axes2d();
    axes
        .set_title("Salesman problem", &[])
        .label(
            &format!("Count: {}", count),
            Coordinate::Graph(0.05),
            Coordinate::Graph(0.95),
            &[],
        )
        .points(
            points.iter().map(|p| p.x),
            points.iter().map(|p| p.y),
            &[
                PlotOption::PointSymbol('O'),
            ]
        );

    let mut prev_point_idx = 0;
    for point_idx in path.path.iter().cycle().skip(1).take(size) {
        let prev_point = &points[prev_point_idx];
        let point = &points[*point_idx];
        axes.arrow(
            Coordinate::Axis(prev_point.x),
            Coordinate::Axis(prev_point.y),
            Coordinate::Axis(point.x),
            Coordinate::Axis(point.y),
            &[
                PlotOption::ArrowType(ArrowheadType::Open),
                PlotOption::ArrowSize(0.03),
            ]
        );
        prev_point_idx = *point_idx;
    }
    fg.show().unwrap();
    fg.close();
}
