#[macro_use] extern crate rocket;
use rocket::serde::{Serialize, json::Json};
use rocket::fairing::AdHoc;
use rocket::serde::Deserialize;

#[post("/ingredients/<ingredient>")]
async fn add_ingredient(ingredient: &str) {
  println!("Add {} to database", ingredient);
}

#[get("/ingredients/<hint>")]
async fn get_ingredients(hint: &str) -> Json<Vec<String>> {
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
    .mount("/", routes![get_ingredients, add_ingredient])
}

#[cfg(test)]
mod test {
  use super::rocket;
  use rocket::local::blocking::Client;
  use rocket::http::Status;

  #[test]
  fn test_get_ingredients() {
    let client = Client::tracked(rocket()).expect("valid rocket instance");
    let response = client.get(uri!(super::get_ingredients("tom"))).dispatch();
    assert_eq!(response.status(), Status::Ok);
    let ingredients = response.into_json::<Vec<String>> ();
    println!("{:?}", ingredients);
    assert!(ingredients.is_some());
  }

#[test]
  fn test_add_ingredient() {
    let client = Client::tracked(rocket()).expect("valid rocket instance");
    let response = client.get(uri!(super::add_ingredient("tomate"))).dispatch();
    assert_eq!(response.status(), Status::Ok);
  }
}
