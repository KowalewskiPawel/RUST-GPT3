#[macro_use] extern crate rocket;
use dotenv::dotenv;

mod api_v1;


#[launch]
fn rocket() -> _ {
    dotenv().ok();
    rocket::build().mount("/api/", routes![api_v1::test, api_v1::send_rq])
}