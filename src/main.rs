use std::convert::{Infallible};

use warp::{Filter, Rejection, Reply};
use warp::http::StatusCode;

const PORT: u16 = 7331;

async fn handle_rejection(error: Rejection) -> Result<impl Reply, Infallible> {
    let (code, message) = if error.is_not_found() {
        (StatusCode::NOT_FOUND, "Resource not found.")
    } else {
        (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error.")
    };

    Ok(warp::reply::with_status(warp::reply::html(message), code))
}

#[tokio::main]
async fn main() {
    let index = warp::path::end().map(|| "https://github.com/statikksh/statikk");
    let routes = warp::get().and(index).recover(handle_rejection);

    println!("The server is listening at port {}.", PORT);
    warp::serve(routes).run(([127, 0, 0, 1], PORT)).await;
}
