use criterion::{BatchSize, Criterion};
use jam::extensions::WriteExt;

pub(crate) fn benchmark(criterion: &mut Criterion) {
    // Write small unsigned varints

    criterion.bench_function("write 1000 varints (small unsigned)", |bencher| {
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
}
