use criterion::{Criterion, criterion_group, criterion_main};
use std::time::Duration;

fn benchmark(criterion: &mut Criterion) {
    // Writing

    write_small_signed::benchmark(criterion);
    write_random_signed::benchmark(criterion);

    write_small_unsigned::benchmark(criterion);
    write_random_unsigned::benchmark(criterion);

    // Reading

    read_small_signed::benchmark(criterion);
    read_random_signed::benchmark(criterion);

    read_small_unsigned::benchmark(criterion);
    read_random_unsigned::benchmark(criterion);
}

fn configuration() -> Criterion {
    Criterion::default()
        .measurement_time(Duration::from_secs(5))
        .sample_size(200)
}

criterion_group! {
    name = benches;
    config = configuration();
    targets = benchmark
}

criterion_main!(benches);

mod read_random_signed;
mod read_random_unsigned;
mod read_small_signed;
mod read_small_unsigned;
mod write_random_signed;
mod write_random_unsigned;
mod write_small_signed;
mod write_small_unsigned;
