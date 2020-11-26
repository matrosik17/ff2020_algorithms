use rand::prelude::SeedableRng;
use rand::rngs::StdRng;
use rand::distributions::{Distribution, Uniform};

use gnuplot::{Figure, PlotOption};


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


fn main() {
    let points = generate_points(1_000, 42);

    let mut fg = Figure::new();
    fg.axes2d().points(
        points.iter().map(|p| p.x),
        points.iter().map(|p| p.y),
        &[
            PlotOption::PointSymbol('O')
        ]
    );
    fg.show().unwrap();
}
