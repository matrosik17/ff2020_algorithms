use rand::prelude::*;
use rand::rngs::StdRng;


fn bubble_sort_iter<T: Ord>(arr: &mut [T]) {
    for i in 0..arr.len()-1 {
        if arr[i] > arr[i+1] {
            arr.swap(i, i+1);
        }
    }
}


fn contra_bubble_sort_iter<T: Ord>(arr: &mut [T]) {
    for i in 0..arr.len()-1 {
        if arr[i] < arr[i+1] {
            arr.swap(i, i+1);
        }
    }
}


pub fn generate_ord_collection(arr_size: usize, n_ord_points: usize, seed: u64) -> Vec<Vec<i64>> {
    let step_size = (arr_size / n_ord_points) as usize;
    let mut rand_vec: Vec<i64> = (0..(arr_size as i64)).collect();
    let mut rng = StdRng::seed_from_u64(seed);
    rand_vec.shuffle(&mut rng);

    // ordered collection
    let mut ordered_collection = Vec::with_capacity(n_ord_points + 1);
    let mut ord_vec = rand_vec.clone();
    ordered_collection.push(ord_vec.clone());
    for _ in 0..=n_ord_points {
        for _ in 0..step_size {
            bubble_sort_iter(&mut ord_vec);
        }
        ordered_collection.push(ord_vec.clone());
    }

    // contra ordered collection
    let mut contra_ordered_collection = Vec::with_capacity(n_ord_points);
    let mut contra_ord_vec = rand_vec.clone();
    for _ in 0..n_ord_points {
        for _ in 0..step_size {
           contra_bubble_sort_iter(&mut contra_ord_vec);
        }
        contra_ordered_collection.push(contra_ord_vec.clone());
    }

    // concat results
    let contra_ord_iter = contra_ordered_collection.into_iter().rev();
    let ord_iter = ordered_collection.into_iter();
    let collection: Vec<Vec<i64>> = contra_ord_iter.chain(ord_iter).collect();
    collection
}