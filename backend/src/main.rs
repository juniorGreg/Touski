#[macro_use] extern crate rocket;
use rocket::serde::{Serialize, json::Json};

#[get("/ingredients")]
fn index() -> Json<Vec<String>> {
    let ingredients = vec!["tomate".to_owned(), "concombre".to_owned(), "radis".to_owned(), "pomme".to_owned(), "épinard".to_owned()];
    Json(ingredients)
}

#[launch]
fn rocket() -> _ {
  rocket::build()
    .mount("/", routes![index])
}
