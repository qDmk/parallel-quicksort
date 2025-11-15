use rand::{Rng, SeedableRng, rngs::StdRng};

use std::{fmt::Debug, ops::RangeInclusive};

fn test_on<S: AsMut<[T]>, T: Copy + Send + PartialOrd + Debug>(mut s: S) {
    let s = s.as_mut();

    quicksort::quicksort(s, 0);

    if !s.is_empty() {
        for i in 1..s.len() {
            assert!(s[i - 1] <= s[i]);
        }
    }
}

#[test]
fn empty() {
    test_on([] as [i32; 0]);
}

#[test]
fn ascending() {
    test_on([1, 2, 3, 4, 5]);
}

#[test]
fn descending() {
    test_on([5, 4, 3, 2, 1]);
}

#[test]
fn all_eq() {
    test_on([1, 1, 1, 1, 1]);
}

fn test_random_within(size: usize, repeat: usize, r: RangeInclusive<u32>) {
    let mut v = vec![0; size];
    let mut rng = StdRng::from_os_rng();
    for _ in 0..repeat {
        for x in v.iter_mut() {
            *x = rng.random_range(r.clone());
        }
        test_on(&mut v);
    }
}

#[test]
fn random_highly_repeated() {
    test_random_within(1000, 10_000, 0..=10);
}

fn test_random(size: usize, repeat: usize) {
    let mut v = vec![0; size];
    let mut rng = StdRng::from_os_rng();
    for _ in 0..repeat {
        rng.fill(v.as_mut_slice());
        test_on(&mut v);
    }
}

#[test]
fn random_100() {
    test_random(100, 100_000);
}

#[test]
fn random_1k() {
    test_random(1_000, 10_000);
}

#[test]
fn random_10k() {
    test_random(10_000, 1_000);
}

#[test]
fn random_100k() {
    test_random(100_000, 10);
}

#[test]
fn random_100kk() {
    test_random(100_000_000, 1);
}
