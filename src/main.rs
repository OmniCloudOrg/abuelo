use routes::get_routes;

mod account;
mod database;
mod handlers;
mod routes;

#[tokio::main]
async fn main() {
    warp::serve(get_routes()).run(([127, 0, 0, 1], 3030)).await;
}
