extern crate criterion;
extern crate edit_distance;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use edit_distance::edit_distance_sequences;

pub fn benchmark_edit_distance_sequences(c: &mut Criterion) {
    let b = "sitting".chars().collect::<Vec<_>>();
    let a = "kitten".chars().collect::<Vec<_>>();

    c.bench_function("edit_distance_sequences", |bencher| {
        bencher.iter(|| edit_distance_sequences(black_box(&a), black_box(&b)))
    });
}

criterion_group!(benches, benchmark_edit_distance_sequences);
criterion_main!(benches);
