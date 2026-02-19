use criterion::{BatchSize, Criterion};
use jam::extensions::{ReadExt, WriteExt};
use std::hint::black_box;

pub(crate) fn benchmark(criterion: &mut Criterion) {
    // Read random signed varints

    criterion.bench_function("read 1000 varints (random signed)", |bencher| {
        bencher.iter_batched(
            || {
                let mut buffer = Vec::with_capacity(16384);

                for _ in 0..1000 {
                    buffer.write_signed_varint(rand::random()).unwrap();
                }

                buffer
            },
            |buffer| {
                let mut slice = buffer.as_slice();

                for _ in 0..1000 {
                    black_box(slice.read_signed_varint().unwrap());
                }
            },
            BatchSize::SmallInput,
        );
    });
}
