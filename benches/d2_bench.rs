use aoc2024::day2;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("d2_1", |b| b.iter(|| day2::part1(black_box(day2::INPUT))));
    c.bench_function("d2_2", |b| b.iter(|| day2::part2(black_box(day2::INPUT))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
