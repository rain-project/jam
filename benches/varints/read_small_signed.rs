use criterion::{BatchSize, Criterion};
use jam::extensions::{ReadExt, WriteExt};
use std::hint::black_box;

pub(crate) fn benchmark(criterion: &mut Criterion) {
    // Read small signed varints

    criterion.bench_function("read 1000 varints (small signed)", |bencher| {
        bencher.iter_batched(
            || {
                let mut buffer = Vec::with_capacity(16384);

                for value in -500..500 {
                    buffer.write_signed_varint(value).unwrap();
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
