use routes::get_routes;

mod account;
mod database;
mod handle;
mod logger;
mod routes;

#[tokio::main]
async fn main() {
    let ret = logger::init();
    match ret {
        Ok(())=>{
            log::info!("Functional log!")
        },
        Err(a)=>{
            println!("Broken log. {}", a)
        },
    }
    warp::serve(get_routes()).run(([127, 0, 0, 1], 3030)).await;
}
