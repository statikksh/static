use warp::Filter;

const PORT: u16 = 7331;

#[tokio::main]
async fn main() {
    let index = warp::get().map(|| "https://github.com/statikksh");

    println!("The server is listening at port {}.", PORT);
    warp::serve(index).run(([127, 0, 0, 1], PORT)).await;
}
