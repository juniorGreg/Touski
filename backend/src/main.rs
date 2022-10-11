#[macro_use] extern crate rocket;
use rocket::serde::{Serialize, json::Json};

#[get("/ingredients/<hint>")]
fn ingredients(hint: &str) -> Json<Vec<String>> {
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
    .mount("/", routes![ingredients])
}

#[cfg(test)]
mod test {
  use super::rocket;
  use rocket::local::blocking::Client;
  use rocket::http::Status;

  #[test]
  fn test_ingredients() {
    let client = Client::tracked(rocket()).expect("valid rokcet instance");
    let mut response = client.get(uri!(super::ingredients("tom"))).dispatch();
    assert_eq!(response.status(), Status::Ok)
  }
}
