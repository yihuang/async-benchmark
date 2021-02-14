use async_benchmark::{smol_timers, tokio_timers};
use std::time::Duration;

fn main() {
    println!("warmup");
    let tokio_rt = tokio::runtime::Builder::new_multi_thread()
        .enable_time()
        .build()
        .unwrap();
    tokio_rt.block_on(tokio_timers(10));
    smol::block_on(smol_timers(10));

    std::thread::sleep(Duration::new(1, 0));

    println!("tokio");
    let iterations = 1000000;
    tokio_rt.block_on(tokio_timers(iterations));

    std::mem::drop(tokio_rt);
    std::thread::sleep(Duration::new(1, 0));

    println!("smol");
    smol::block_on(smol_timers(iterations));
}
