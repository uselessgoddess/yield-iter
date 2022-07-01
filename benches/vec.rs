#![feature(generators)]

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use yield_iter::generator;

fn generator_vs_vector(c: &mut Criterion) {
    let data = vec![228; 177013];

    c.bench_function("iter", |b| {
        b.iter(|| {
            data.iter().for_each(|x| {
                black_box(*x);
            });
        })
    });

    c.bench_function("generator", |b| {
        b.iter(|| {
            let mut iter = generator! {
                for x in &data {
                    yield x
                }
            };
            iter.for_each(|x| {
                black_box(*x);
            });
        })
    });

    c.bench_function("vector", |b| {
        b.iter(|| {
            let mut vec: Vec<_> = data.iter().collect();
            for x in vec {
                black_box(*x);
            }
        })
    });
}

criterion_group!(benches, generator_vs_vector);
criterion_main!(benches);
