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

    #[cfg(feature = "y2022d03")]
    {
        use narigama_aoc::y2022::d03 as target;
        let input = target::get_input().unwrap();

        c.bench_function("y2022d03p01", |b| {
            b.iter(|| target::part_one(black_box(&input)).unwrap())
        });

        c.bench_function("y2022d03p02", |b| {
            b.iter(|| target::part_two(black_box(&input)).unwrap())
        });
    }

    #[cfg(feature = "y2022d04")]
    {
        use narigama_aoc::y2022::d04 as target;
        let input = target::get_input().unwrap();

        c.bench_function("y2022d04p01", |b| {
            b.iter(|| target::part_one(black_box(&input)))
        });

        c.bench_function("y2022d04p02", |b| {
            b.iter(|| target::part_two(black_box(&input)))
        });
    }

    #[cfg(feature = "y2022d05")]
    {
        use narigama_aoc::y2022::d05 as target;
        let input = target::get_input().unwrap();

        c.bench_function("y2022d05p01", |b| {
            b.iter(|| target::part_one(black_box(&mut input.clone())).unwrap())
        });

        c.bench_function("y2022d05p02", |b| {
            b.iter(|| target::part_two(black_box(&mut input.clone())).unwrap())
        });
    }

    #[cfg(feature = "y2022d06")]
    {
        use narigama_aoc::y2022::d06 as target;
        let input = target::get_input().unwrap();

        c.bench_function("y2022d06p01", |b| {
            b.iter(|| target::part_one(black_box(&mut input.clone())))
        });

        c.bench_function("y2022d06p02", |b| {
            b.iter(|| target::part_two(black_box(&mut input.clone())))
        });
    }

    #[cfg(feature = "y2022d07")]
    {
        use narigama_aoc::y2022::d07 as target;
        let input = target::get_input().unwrap();

        c.bench_function("y2022d07p01", |b| {
            b.iter(|| target::part_one(black_box(&input)))
        });

        c.bench_function("y2022d07p02", |b| {
            b.iter(|| target::part_two(black_box(&input)))
        });
    }

    #[cfg(feature = "y2022d08")]
    {
        use narigama_aoc::y2022::d08 as target;
        let input = target::get_input().unwrap();

        c.bench_function("y2022d08p01", |b| {
            b.iter(|| target::part_one(black_box(&input)))
        });

        c.bench_function("y2022d08p02", |b| {
            b.iter(|| target::part_two(black_box(&input)))
        });
    }

    #[cfg(feature = "y2022d09")]
    {
        use narigama_aoc::y2022::d09 as target;
        let input = target::get_input().unwrap();

        c.bench_function("y2022d09p01", |b| {
            b.iter(|| target::part_one(black_box(&input)))
        });

        c.bench_function("y2022d09p02", |b| {
            b.iter(|| target::part_two(black_box(&input)))
        });
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
