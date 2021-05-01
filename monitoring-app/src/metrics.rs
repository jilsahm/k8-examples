use std::convert::Infallible;

use lazy_static::lazy_static;
use prometheus::{Encoder, IntCounter, TextEncoder, labels, opts, register_int_counter};
use warp::{Filter, hyper::Response};

lazy_static! {
    pub static ref SUCCESS_EVENT_COUNTER : IntCounter = register_int_counter!(
        opts!(
            "monitoring_event_counter",
            "Number of sample events",
            labels! {"type" => "success"}
        )
    ).unwrap();

    pub static ref FAILURE_EVENT_COUNTER : IntCounter = register_int_counter!(
        opts!(
            "monitoring_event_counter",
            "Number of sample events",
            labels! {"type" => "failure"}
        )
    ).unwrap();
}

pub fn create_route() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("metrics")
        .and(warp::get())
        .and_then(metrics_handler)
}

async fn metrics_handler() -> Result<impl warp::Reply, Infallible> {
    Ok(Response::builder().body(gather_metrics()))   
}

fn gather_metrics() -> Vec<u8> {
    let encoder = TextEncoder::new();
    let metric_families = prometheus::gather();
    let mut buffer = vec![];
    encoder.encode(&metric_families, &mut buffer).unwrap();
    buffer
}

#[cfg(test)]
mod tests {
    use super::{FAILURE_EVENT_COUNTER, SUCCESS_EVENT_COUNTER, create_route, gather_metrics};
    use warp::{hyper::StatusCode, test::request};

    #[test]
    fn test_gather_metrics() {
        SUCCESS_EVENT_COUNTER.inc_by(42);
        FAILURE_EVENT_COUNTER.inc_by(120);
        let result = String::from_utf8_lossy(&gather_metrics()).to_string();
        let expected = "# HELP monitoring_event_counter Number of sample events\n# TYPE monitoring_event_counter counter\nmonitoring_event_counter{type=\"failure\"} 120\nmonitoring_event_counter{type=\"success\"} 42\n";
        assert_eq!(expected, result);
    }

    #[tokio::test]
    async fn test_metrics_route() {
        let api = create_route();
        let resp = request()
            .method("GET")
            .path("/metrics")
            .reply(&api)
            .await;
        assert_eq!(StatusCode::OK, resp.status());
    }
}