use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    #[cfg(feature = "y2022d01")]
    {
        use narigama_aoc::y2022::d01 as target;
        let input = target::get_input().unwrap();

        c.bench_function("y2022d01p01", |b| {
            b.iter(|| target::part_one(black_box(&input)))
        });

        c.bench_function("y2022d01p02", |b| {
            b.iter(|| target::part_two(black_box(&input)))
        });
    }

    #[cfg(feature = "y2022d02")]
    {
        use narigama_aoc::y2022::d02 as target;
        let input = target::get_input().unwrap();

        c.bench_function("y2022d02p01", |b| {
            b.iter(|| target::part_one(black_box(&input)))
        });

        c.bench_function("y2022d02p02", |b| {
            b.iter(|| target::part_two(black_box(&input)))
        });
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
