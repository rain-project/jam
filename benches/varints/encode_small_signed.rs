use criterion::{BatchSize, Criterion};
use jam::extensions::WriteExt;

pub(crate) fn benchmark(criterion: &mut Criterion) {
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
}
