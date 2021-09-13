use criterion::BenchmarkId;
use criterion::Criterion;
use criterion::{criterion_group, criterion_main};
use rust_counter_strings::generate;

fn bench(c: &mut Criterion) {
    let count: usize = 100;

    c.bench_with_input(BenchmarkId::new("generate", count), &count, |b, &s| {
        b.iter(|| generate(s));
    });
}

criterion_group!(benches, bench);
criterion_main!(benches);
