#[macro_use] extern crate rocket;
use dotenv::dotenv;

mod api_v1;
mod appconfig;

#[launch]
fn rocket() -> _ {
    dotenv().ok();
    appconfig::check_dbfile(appconfig::DATABASE_FILE);
    rocket::build().mount("/api/", routes![api_v1::test, api_v1::send_rq, api_v1::create_key, api_v1::query_all])
}