use routes::get_routes;
mod account;
mod database;
mod handle;
mod logger;
mod routes;
pub mod mfa;

use totp_rs::Secret;
use totp_rs::TOTP;
use totp_rs::Algorithm;

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

    let totp = TOTP::new(
        Algorithm::SHA1,
        6,
        1,
        30,
        Secret::Encoded("KRSXG5CTMVRXEZLUKN2XAZLSKNSWG4TFOQ".to_string()).to_bytes().unwrap(),
    ).unwrap();
    let token = totp.generate_current().unwrap();
    println!("{}", token);

 let _ = rocket::build()
        .mount("/", routes::get_routes())
        .launch()
        .await;



   
    // warp::serve(get_routes()).run(([127, 0, 0, 1], 3030)).await;
}
