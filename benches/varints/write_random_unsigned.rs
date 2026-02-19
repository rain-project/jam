use criterion::{BatchSize, Criterion};
use jam::extensions::WriteExt;
use std::iter;

pub(crate) fn benchmark(criterion: &mut Criterion) {
    // Write random unsigned varints

    criterion.bench_function("write 1000 varints (random unsigned)", |bencher| {
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
