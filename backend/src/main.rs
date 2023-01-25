#[macro_use] extern crate rocket;
use rocket::http::Header;
use rocket::{Request, Response};
use rocket::fairing::{Fairing, Info, Kind};
use dotenv::dotenv;
use rocket::http::Method;
use rocket_cors::{AllowedOrigins, CorsOptions};

mod api_v1;
mod appconfig;

#[launch]
fn rocket() -> _ {
    let cors = CorsOptions::default()
    .allowed_origins(AllowedOrigins::all())
    .allowed_methods(
        vec![Method::Get, Method::Post, Method::Patch]
            .into_iter()
            .map(From::from)
            .collect(),
    )
    .allow_credentials(true);
    
    dotenv().ok();
    appconfig::check_dbfile(appconfig::DATABASE_FILE);
    rocket::build().attach(cors.to_cors().unwrap()).mount("/api/", routes![api_v1::test, api_v1::send_rq, api_v1::create_key, api_v1::query_all])
}