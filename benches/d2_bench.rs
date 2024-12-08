use aoc2024::day7::*;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("d2_1", |b| b.iter(|| part1(black_box(INPUT))));
    c.bench_function("d2_2", |b| b.iter(|| part2(black_box(INPUT))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
