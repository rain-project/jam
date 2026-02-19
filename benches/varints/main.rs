use criterion::{Criterion, criterion_group, criterion_main};

fn benchmark(criterion: &mut Criterion) {
    // Encoding

    encode_small_signed::benchmark(criterion);
    encode_random_signed::benchmark(criterion);

    encode_small_unsigned::benchmark(criterion);
    encode_random_unsigned::benchmark(criterion);
}

criterion_group!(benches, benchmark);
criterion_main!(benches);

mod encode_random_signed;
mod encode_random_unsigned;
mod encode_small_signed;
mod encode_small_unsigned;
