use std::time::Duration;
use futures_lite::StreamExt;

pub async fn smol_timers(n: u64) {
    let mut timer = smol::Timer::interval(Duration::new(0, 0));
    for _ in 0..n {
        let _ = timer.next().await;
    }
}

pub async fn tokio_timers(n: u64) {
    let mut timer = tokio::time::interval(Duration::new(0, 0));
    for _ in 0..n {
        let _ = timer.tick().await;
    }
}
