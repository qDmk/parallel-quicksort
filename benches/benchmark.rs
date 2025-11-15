use criterion::{BatchSize, Criterion, criterion_group, criterion_main};
use quicksort::quicksort;
use rand::{SeedableRng, rngs::StdRng};

fn random_vec_generator(mut rng: impl rand::Rng, size: usize) -> impl FnMut() -> Vec<i32> {
    move || -> Vec<i32> {
        let mut v = vec![0; size];
        rng.fill(v.as_mut_slice());
        v
    }
}

pub fn criterion_benchmark(c: &mut Criterion) {
    let small_rng = StdRng::from_seed(rand::random());
    let size = 100_000_000;

    c.bench_function("quicksort x4", |b| {
        b.iter_batched(
            random_vec_generator(small_rng.clone(), size),
            |mut slice| quicksort(&mut slice, 2),
            BatchSize::SmallInput,
        )
    });

    c.bench_function("quicksort x1", |b| {
        b.iter_batched(
            random_vec_generator(small_rng.clone(), size),
            |mut slice| quicksort(&mut slice, 0),
            BatchSize::SmallInput,
        )
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(10);
    targets = criterion_benchmark
}
criterion_main!(benches);
