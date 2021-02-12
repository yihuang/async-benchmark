use std::time::Duration;

use criterion::{criterion_group, criterion_main, Criterion};
use smol::{Timer, block_on};
use futures_lite::StreamExt;

async fn smol_timers(n: u64) {
    let mut timer = Timer::interval(Duration::new(0, 0));
    for _ in 0..n {
        let _ = timer.next().await;
    }
}

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("smol timers", |b| b.iter(|| block_on(smol_timers(1000))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
