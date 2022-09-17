#[macro_use] extern crate rocket;
use rocket::serde::{Serialize, json::Json};

#[get("/ingredients/<hint>")]
fn index(hint: &str) -> Json<Vec<String>> {
    let mut ingredients = vec!["tomate".to_owned(), 
                               "concombre".to_owned(), 
                               "radis".to_owned(), 
                               "pomme".to_owned(), 
                               "épinard".to_owned(),
                               "épinette".to_owned(),
                               "patate".to_owned(),
                               "banane".to_owned(),
                               "pitahaya".to_owned()];
    ingredients.retain(|word| word.starts_with(hint));

    Json(ingredients)
}

#[launch]
fn rocket() -> _ {
  rocket::build()
    .mount("/", routes![index])
}
