extern crate criterion;
extern crate edit_distance;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use edit_distance::edit_distance;

pub fn benchmark_edit_distance(c: &mut Criterion) {
    let a = "kitten";
    let b = "sitting";

    c.bench_function("edit_distance", |bencher| {
        bencher.iter(|| edit_distance(black_box(a), black_box(b)))
    });
}

criterion_group!(benches, benchmark_edit_distance);
criterion_main!(benches);
