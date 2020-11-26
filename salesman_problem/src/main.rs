use rand::prelude::SeedableRng;
use rand::rngs::StdRng;
use rand::distributions::{Distribution, Uniform};

use gnuplot::{Figure, PlotOption, Coordinate};


#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn new(x: f64, y: f64) -> Self {
        Self{x, y}
    }
}


fn dist(a: &Point, b: &Point) -> f64 {
    let dist2 = (a.x - b.x).powi(2) + (a.y - b.y).powi(2);
    dist2.sqrt()
}


fn generate_points(size: usize, seed: u64) -> Vec<Point> {
    let mut rng = StdRng::seed_from_u64(seed);
    let uniform = Uniform::new(0., 1.);

    (0..size)
        .map(|_| {
            let x = uniform.sample(&mut rng);
            let y = uniform.sample(&mut rng);
            Point::new(x, y)
        })
        .collect()
}


#[derive(Clone, Debug)]
struct Path {
    path: Vec<usize>,
    length: f64,
    path_length: usize,
}

impl Path {

    fn new(size: usize, path_length: usize, points: &[Point]) -> Self {
        let path: Vec<usize> = (0..size).collect();

        let mut length = 0.;
        if path_length > 1 {
            let mut prev_idx = 0;
            for idx in path.iter().take(path_length).skip(1) {
                length += dist(&points[prev_idx], &points[*idx]);
                prev_idx = *idx;
            }
        }

        Self {
            path,
            length,
            path_length,
        }
    }

    fn add(&mut self, idx: usize, points: &[Point]) {
        if idx < self.path_length { panic!("Wrong path idx - element already set"); }
        if idx >= self.path.len() { panic!("Wrong path idx - out of bounds"); }

        self.path.swap(self.path_length, idx);
        let prev_point_idx = self.path[self.path_length - 1];
        let point_idx = self.path[self.path_length];
        self.length += dist(&points[prev_point_idx], &points[point_idx]);
        self.path_length += 1;
    }

}

fn find_optimal_path_impl(curr_path: Path, best_path: &mut Path, points: &[Point]) -> usize {
    if curr_path.path_length < best_path.path_length {
        let mut count = 0;
        for (idx, _) in curr_path.path.iter().enumerate().skip(curr_path.path_length) {
            let mut path = curr_path.clone();
            path.add(idx, points);
            if path.length < best_path.length {
                count += find_optimal_path_impl(path, best_path, points);
            } else {
                count += 1;
            }
            // count += find_optimal_path_impl(path, best_path, points);
        }
        return count;
    } else {
        if curr_path.length < best_path.length {
            *best_path = curr_path.clone();
        }
        return 1;
    }
}

fn find_optimal_path(points: &[Point]) -> (Path, usize) {
    let size = points.len();
    let mut best_path = Path::new(size, size, points);
    let curr_path = Path::new(size, 1, points);
    let count = find_optimal_path_impl(curr_path, &mut best_path, points);
    (best_path, count)
}


fn main() {
    let size = 12;
    let points = generate_points(size, 42);
    let (path, count) = find_optimal_path(&points);
    println!("points: {:?}\ncount: {}", points, count);

    let mut fg = Figure::new();
    let axes = fg.axes2d();
    axes.points(
        points.iter().map(|p| p.x),
        points.iter().map(|p| p.y),
        &[
            PlotOption::PointSymbol('O')
        ]
    );

    let mut prev_point_idx = 0;
    for point_idx in path.path.iter().skip(1) {
        let prev_point = &points[prev_point_idx];
        let point = &points[*point_idx];
        axes.arrow(
            Coordinate::Axis(prev_point.x),
            Coordinate::Axis(prev_point.y),
            Coordinate::Axis(point.x),
            Coordinate::Axis(point.y),
            &[]
        );
        prev_point_idx = *point_idx;
    }
    fg.show().unwrap();
}
