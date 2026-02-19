use std::iter;

use criterion::{BatchSize, Criterion};
use jam::extensions::WriteExt;

pub(crate) fn benchmark(criterion: &mut Criterion) {
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
}
