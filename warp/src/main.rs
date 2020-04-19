use warp::Filter;
use std::sync::{Arc};
use tokio::sync::{Mutex};

#[tokio::main]
async fn main() {

    // Application status
    let db = Arc::new(Mutex::new(0 as u8));
    let db = warp::any().map(move || db.clone());

    // Match any request and return hello world!
    let hello = warp::get().and(warp::path("hello")).and_then(hello);
    let counter = warp::get().and(warp::path("counter")).and(db.clone()).and_then(counter);

    let routes = hello.or(counter);
    warp::serve(routes).run(([0, 0, 0, 0], 3030)).await;
}

async fn hello() -> Result<impl warp::Reply, warp::Rejection> {
    Ok("Hello, World!")
}

async fn counter(db: Arc<Mutex<u8>>) -> Result<impl warp::Reply, warp::Rejection> {
    let mut counter = db.lock().await;
    *counter += 1;
    Ok(counter.to_string())
}