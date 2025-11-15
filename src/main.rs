use rand::{Rng, SeedableRng, rngs::SmallRng};

fn main() {
    let mut rng = rand::rng();
    let small_rng = SmallRng::from_seed(rng.random());

    let random_slice = || -> Vec<i32> {
        let mut local_rng = small_rng.clone();
        std::iter::repeat_with(|| local_rng.random())
            .take(100_000_000)
            .collect()
    };

    let mut v = random_slice();
    let (v1, v2) = quicksort::split(&mut v).unwrap();
    println!("ratio 1: {}", v1.len() as f64 / v2.len() as f64);

    let (v11, v12) = quicksort::split(v1).unwrap();
    println!("ratio 2.1: {}", v11.len() as f64 / v12.len() as f64);

    let (v21, v22) = quicksort::split(v2).unwrap();
    println!("ratio 2.2: {}", v21.len() as f64 / v22.len() as f64);
}
