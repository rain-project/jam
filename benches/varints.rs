use criterion::{BatchSize, Criterion, criterion_group, criterion_main};
use jam::extensions::WriteExt;
use std::iter;

fn benchmark(criterion: &mut Criterion) {
    // Encode small signed varints

    criterion.bench_function("encode 1000 varints (small signed)", |bencher| {
        bencher.iter_batched(
            || Vec::with_capacity(16384),
            |mut buffer| {
                for value in -500..500 {
                    buffer.write_signed_varint(value).unwrap();
                }

                buffer
            },
            BatchSize::SmallInput,
        );
    });

    // Encode random signed varints

    criterion.bench_function("encode 1000 varints (random signed)", |bencher| {
        bencher.iter_batched(
            || {
                (
                    Vec::with_capacity(16384),
                    iter::repeat_with(rand::random::<i64>)
                        .take(1000)
                        .collect::<Vec<_>>(),
                )
            },
            |(mut buffer, values)| {
                for value in values {
                    buffer.write_signed_varint(value).unwrap();
                }

                buffer
            },
            BatchSize::SmallInput,
        );
    });

    // Encode small unsigned varints

    criterion.bench_function("encode 1000 varints (small unsigned)", |bencher| {
        bencher.iter_batched(
            || Vec::with_capacity(16384),
            |mut buffer| {
                for value in 0..1000 {
                    buffer.write_unsigned_varint(value).unwrap();
                }

                buffer
            },
            BatchSize::SmallInput,
        );
    });

    // Encode random unsigned varints

    criterion.bench_function("encode 1000 varints (random unsigned)", |bencher| {
        bencher.iter_batched(
            || {
                (
                    Vec::with_capacity(16384),
                    iter::repeat_with(rand::random::<u64>)
                        .take(1000)
                        .collect::<Vec<_>>(),
                )
            },
            |(mut buffer, values)| {
                for value in values {
                    buffer.write_unsigned_varint(value).unwrap();
                }

                buffer
            },
            BatchSize::SmallInput,
        );
    });
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
