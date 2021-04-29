use std::{cmp::Ordering, convert::Infallible, time::SystemTime};

use warp::{Filter, hyper::StatusCode};

pub fn create_route(delay: u64) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let start = SystemTime::now();
    warp::path("ready")
        .and(warp::get())
        .and(warp::any().map(move || start.clone()))
        .and(warp::any().map(move || delay.clone()))
        .and_then(ready_handler)
}

async fn ready_handler(start: SystemTime, delay: u64) -> Result<impl warp::Reply, Infallible> {
    match start.elapsed().unwrap_or_default().as_secs().cmp(&delay) {
        Ordering::Less => Ok(warp::reply::with_status(warp::reply(), StatusCode::INTERNAL_SERVER_ERROR)),
        _ => Ok(warp::reply::with_status(warp::reply(), StatusCode::OK))
    }    
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use warp::{hyper::StatusCode, test::request};

    use super::create_route;

    #[tokio::test]
    async fn test_ready() {
        let api = create_route(0);
        let resp = request()
            .method("GET")
            .path("/ready")
            .reply(&api)
            .await;
        assert_eq!(StatusCode::OK, resp.status());
    }

    #[tokio::test]
    async fn test_not_ready() {
        let api = create_route(9000);
        let resp = request()
            .method("GET")
            .path("/ready")
            .reply(&api)
            .await;
        assert_eq!(StatusCode::INTERNAL_SERVER_ERROR, resp.status());
    }

    #[tokio::test]
    async fn test_swap_ready() {
        let api = create_route(1);
        let resp = request()
            .method("GET")
            .path("/ready")
            .reply(&api)
            .await;
        assert_eq!(StatusCode::INTERNAL_SERVER_ERROR, resp.status());

        tokio::time::sleep(Duration::from_secs(1)).await;
        let resp = request()
            .method("GET")
            .path("/ready")
            .reply(&api)
            .await;
        assert_eq!(StatusCode::OK, resp.status());
    }
}