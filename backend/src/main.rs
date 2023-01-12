#[macro_use] extern crate rocket;

mod api_v1;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/api/", routes![api_v1::test])
}