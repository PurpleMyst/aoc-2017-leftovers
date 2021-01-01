use std::time::Duration;

use criterion::{criterion_group, criterion_main, BatchSize, Criterion};

pub fn day20_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("day20");

    group.bench_function("parsing", |b| b.iter(day20::load_input));

    let input = day20::load_input();

    group.bench_function("part1", |b| b.iter(|| day20::solve_part1(&input)));

    group.bench_function("part2", |b| {
        b.iter_batched_ref(
            || input.clone(),
            |input| day20::solve_part2(input),
            BatchSize::SmallInput,
        )
    });

    group.bench_function("solve", |b| b.iter(day20::solve));

    group.finish()
}

pub fn day23_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("day23");

    group.bench_function("parsing", |b| b.iter(day23::load_input));

    group.bench_function("part1", |b| {
        b.iter_batched_ref(
            day23::load_input,
            |input| day23::solve_part1(input),
            BatchSize::SmallInput,
        )
    });

    group.bench_function("part2", |b| {
        b.iter_batched_ref(
            day23::load_input,
            |input| day23::solve_part2(input),
            BatchSize::SmallInput,
        )
    });

    group.bench_function("solve", |b| b.iter(day23::solve));

    group.finish()
}

pub fn day21_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("day21");

    group.bench_function("solve", |b| b.iter(day21::solve));

    group.finish()
}

criterion_group! {
    name = benches;

    config = Criterion::default()
        .significance_level(0.1)
        .sample_size(350)
        .measurement_time(Duration::from_secs(10))
        .warm_up_time(Duration::from_secs(5))
        .noise_threshold(0.05);

    targets =
        day20_benchmark,
        day21_benchmark,
        day23_benchmark,
}

criterion_main!(benches);
