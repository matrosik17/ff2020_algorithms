use rand::prelude::*;

pub(crate) fn sort_determenistic_test<F: Fn(&mut [i64])>(sort: F) {
    let mut test_vec = vec![5, 1, 2, 4, 1];
    let expected_vec = vec![1, 1, 2, 4, 5];

    sort(&mut test_vec);
    assert_eq!(test_vec, expected_vec);
}

pub(crate) fn sort_random_test<F: Fn(&mut [i64])>(sort: F) {
    let n_samples = 100;
    let arr_len = 100;
    let expected_vec: Vec<i64> = (0i64..arr_len).map(i64::from).collect();
    let mut rng = thread_rng();

    for _ in 0..n_samples {
        let mut test_vec = expected_vec.clone();
        test_vec.shuffle(&mut rng);

        sort(&mut test_vec);
        assert_eq!(test_vec, expected_vec);
    }
}
