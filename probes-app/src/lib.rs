#[macro_use]
extern crate log;

use config::Configuration;
use warp::Filter;

pub mod config;
mod ready;

pub async fn run(config: Configuration) {
    info!("Starting app, ready in {} seconds, faulty in {} seconds", config.seconds_till_ready, config.seconds_till_faulty);
    let router = ready::create_route(config.seconds_till_ready).with(warp::log("info"));
    warp::serve(router).run(([0, 0, 0, 0], 8080)).await;
}
