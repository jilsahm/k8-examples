use std::time::Duration;
use std::thread;

use rand::Rng;
use warp::Filter;

#[macro_use]
extern crate log;

mod metrics;

fn random_events() {
    let mut rng = rand::thread_rng();
    loop {
        match rng.gen_range(0..3) {
            0 | 1 => {
                info!("Random success event monitored");
                metrics::SUCCESS_EVENT_COUNTER.inc();
            }
            _ => {
                info!("Random failure event monitored");
                metrics::FAILURE_EVENT_COUNTER.inc();
            }
        }
        std::thread::sleep(Duration::from_millis(rng.gen_range(700..2000)));
    }
}

pub async fn run() {
    info!("Starting monitoring app");
    let router = metrics::create_route().with(warp::log("info"));
    thread::spawn(|| random_events());
    warp::serve(router).run(([0, 0, 0, 0], 8080)).await;
}
