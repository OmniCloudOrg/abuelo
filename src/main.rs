use abuelo::{logger, routes};

#[rocket::main]
async fn main() {
    let ret = logger::init();
    match ret {
        Ok(()) => {
            log::info!("Functional log!")
        },
        Err(a) => {
            println!("Broken log. {}", a)
        },
    }
    
    let _ = rocket::build()
        .mount("/", routes::get_routes())
        .launch()
        .await;
}