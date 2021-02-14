use async_benchmark::{smol_timers, tokio_timers};

use criterion::{criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    let iterations = 1000;
    let tokio_rt = tokio::runtime::Builder::new_current_thread()
        .enable_time()
        .build()
        .unwrap();
    c.bench_function("smol timers", |b| {
        b.iter(|| smol::block_on(smol_timers(iterations)))
    });
    c.bench_function("tokio timers", |b| {
        b.iter(|| tokio_rt.block_on(tokio_timers(iterations)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
