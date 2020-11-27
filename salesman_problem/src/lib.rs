use std::collections::HashSet;
use std::cmp::Ordering;

use rand::prelude::SeedableRng;
use rand::rngs::StdRng;
use rand::distributions::{Distribution, Uniform};


#[derive(Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
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


pub fn generate_points(size: usize, seed: u64) -> Vec<Point> {
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
pub struct Path {
    pub path: Vec<usize>,
    pub length: f64,
    pub path_length: usize,
}

impl Path {

    fn new(path_length: usize, points: &[Point]) -> Self {
        let size = points.len();
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

pub fn path_greedy(points: &[Point]) -> Path {
    let mut path = Path::new(1, points);
    let mut prev_point_idx = 0;
    let mut unused_points: HashSet<usize> = (1..points.len()).collect();

    for _ in 1..points.len() {
        let nearest_point_idx = *unused_points.iter()
            .min_by(|&idx1, &idx2| {
                let dist1 = dist(&points[prev_point_idx], &points[*idx1]);
                let dist2 = dist(&points[prev_point_idx], &points[*idx2]);
                match dist1.partial_cmp(&dist2) {
                    Some(ord) => ord,
                    None => Ordering::Greater,
                }
            })
            .unwrap();

        let nearest_point_pos = path.path.iter().position(|x| *x == nearest_point_idx).unwrap();
        path.add(nearest_point_pos, points);

        prev_point_idx = nearest_point_idx;
        unused_points.remove(&nearest_point_idx);
    }
    path
}

fn find_optimal_path_impl(curr_path: Path, best_path: &mut Path, points: &[Point]) -> usize {
    if curr_path.path_length < best_path.path_length {
        let mut count = 0;
        for (idx, _) in curr_path.path.iter().enumerate().skip(curr_path.path_length) {
            let mut path = curr_path.clone();
            path.add(idx, points);
            count += find_optimal_path_impl(path, best_path, points);
        }
        return count;
    } else {
        if curr_path.length < best_path.length {
            *best_path = curr_path.clone();
        }
        return 1;
    }
}

fn find_optimal_path_impl_optimal(curr_path: Path, best_path: &mut Path, points: &[Point]) -> usize {
    if curr_path.path_length < best_path.path_length {
        let mut count = 0;
        for (idx, _) in curr_path.path.iter().enumerate().skip(curr_path.path_length) {
            let mut path = curr_path.clone();
            path.add(idx, points);
            if path.length < best_path.length {
                count += find_optimal_path_impl_optimal(path, best_path, points);
            } else {
                count += 1;
            }
        }
        return count;
    } else {
        if curr_path.length < best_path.length {
            *best_path = curr_path.clone();
        }
        return 1;
    }
}

pub fn find_optimal_path(points: &[Point]) -> (Path, usize) {
    let mut best_path = Path::new(points.len(), points);
    let curr_path = Path::new(1, points);
    let count = find_optimal_path_impl(curr_path, &mut best_path, points);
    (best_path, count)
}

pub fn find_optimal_path_optimal(points: &[Point]) -> (Path, usize) {
    let mut best_path = path_greedy(points);
    let curr_path = Path::new(1, points);
    let count = find_optimal_path_impl_optimal(curr_path, &mut best_path, points);
    (best_path, count)
}
