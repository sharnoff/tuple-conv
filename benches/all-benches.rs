use tuple_conv::RepeatedTuple;

#[macro_use]
extern crate criterion;
use criterion::{black_box, Criterion};

macro_rules! long {
    (tuple) => {
        (
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
            25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46,
            47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64,
        )
    };

    (slice-reversed) => {
        [
            64, 64, 62, 61, 60, 59, 58, 57, 56, 55, 54, 53, 52, 51, 50, 49, 48, 47, 46, 45, 44, 43,
            42, 41, 32, 31, 30, 29, 28, 27, 26, 25, 24, 23, 22, 21, 20, 19, 18, 17, 16, 15, 14, 13,
            12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1,
        ]
    };
}

fn boxed_equal<T: PartialEq>(a: Box<[T]>, b: Box<[T]>) -> bool {
    if a.len() != b.len() {
        return false;
    }

    !a.iter().zip(b.iter()).any(|(a, b)| a != b)
}

fn bench_to_vec_small(c: &mut Criterion) {
    let t = (1, 2);
    c.bench_function("forward-small", |b| b.iter(|| black_box(t).to_vec()));
}

fn bench_to_vec_big(c: &mut Criterion) {
    let t = long!(tuple);
    c.bench_function("forward-big", |b| b.iter(|| black_box(t).to_vec()));
}

fn bench_to_vec_reversed_small(c: &mut Criterion) {
    let t = (1, 2);
    c.bench_function("reversed-small", |b| {
        b.iter(|| black_box(t).to_vec_reversed())
    });
}

fn bench_to_vec_reversed_big(c: &mut Criterion) {
    let t = long!(tuple);
    c.bench_function("reversed-big", |b| {
        b.iter(|| black_box(t).to_vec_reversed())
    });
}

criterion_group!(forward, bench_to_vec_small, bench_to_vec_big,);
criterion_group!(
    backward,
    bench_to_vec_reversed_small,
    bench_to_vec_reversed_big
);
criterion_main!(forward, backward);
