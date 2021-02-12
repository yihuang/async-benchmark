use std::time::Duration;

use criterion::{criterion_group, criterion_main, Criterion};
use futures_lite::StreamExt;

async fn smol_timers(n: u64) {
    let mut timer = smol::Timer::interval(Duration::new(0, 0));
    for _ in 0..n {
        let _ = timer.next().await;
    }
}

async fn tokio_timers(n: u64) {
    let mut timer = tokio::time::interval(Duration::new(0, 0));
    for _ in 0..n {
        let _ = timer.tick().await;
    }
}

pub fn criterion_benchmark(c: &mut Criterion) {
    let iterations = 1000;
    let tokio_rt = tokio::runtime::Builder::new_current_thread().enable_time().build().unwrap();
    c.bench_function("smol timers", |b| b.iter(|| smol::block_on(smol_timers(iterations))));
    c.bench_function("tokio timers", |b| b.iter(|| tokio_rt.block_on(tokio_timers(iterations))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
