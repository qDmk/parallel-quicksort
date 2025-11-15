use std::fmt::Debug;

pub fn find_median<T: Copy + PartialOrd>(s: &mut [T], depth: u32) -> T {
    if depth == 0 {
        return s[0];
    }

    let i = s.len() / 3;

    let (left, right) = s.split_at_mut(i);
    let (mid, right) = right.split_at_mut(i);
    let a = find_median(left, depth - 1);
    let b = find_median(mid, depth - 1);
    let c = find_median(right, depth - 1);

    #[allow(clippy::collapsible_else_if)]
    if a < b {
        if b < c {
            b
        } else if a < c {
            c
        } else {
            a
        }
    } else {
        if c < b {
            b
        } else if c < a {
            c
        } else {
            a
        }
    }
}

pub fn split<T: Copy + PartialOrd + Debug>(s: &mut [T]) -> Option<(&mut [T], &mut [T])> {
    let median_depth = match s.len() {
        0..=1 => return None,
        2 => 0,
        3..50 => 1,
        50..10_000 => 2,
        10_000..10_000_000 => 3,
        _ => 8,
    };
    let median = find_median(s, median_depth);
    let (split_index, lower_i) = split_by(s, median);

    let (left, right) = s.split_at_mut(split_index);
    if !right.is_empty() {
        return Some((left, right));
    }

    match lower_i {
        Some(lower_i) => {
            let (split_index, _) = split_by(left, left[lower_i]);
            Some(left.split_at_mut(split_index))
        }
        None => None,
    }
}

pub fn split_by<T: Copy + PartialOrd + Debug>(s: &mut [T], x: T) -> (usize, Option<usize>) {
    let len = s.len();

    let mut j = 0;
    let mut lower = None;
    for i in 0..len {
        let elem = s[i];
        if elem <= x {
            s.swap(i, j);
            j += 1;
        }
        if elem < x {
            lower = Some(i);
        }
    }
    (j, lower)
}

pub fn quicksort<T: Copy + Send + PartialOrd + Debug>(s: &mut [T], factor: usize) {
    let Some((left, right)) = split(s) else {
        return;
    };
    if factor > 0 {
        unsafe {
            let handle = std::thread::Builder::new()
                .spawn_unchecked(|| {
                    quicksort(left, factor - 1);
                })
                .unwrap();
            quicksort(right, factor - 1);
            handle.join().unwrap();
        }
    } else {
        quicksort(left, 0);
        quicksort(right, 0);
    }
}
