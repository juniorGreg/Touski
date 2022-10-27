#[macro_use] extern crate rocket;
use rocket::serde::{Serialize, json::Json, Deserialize};
use rocket::State;
use diesel::prelude::*;
use std::sync::{Arc, Mutex};
use dotenvy::dotenv;
use std::env;

pub mod models;
pub mod schema;

use models::*;
use crate::Ingredient;

struct DieselConnection {
  conn: Arc::<Mutex<PgConnection>>
}

fn connect_db() -> PgConnection {
  dotenv().ok();
  let database_url= env::var("DATABASE_URL").expect("DATABASE_URL must be set");
  PgConnection::establish(&database_url)
    .unwrap_or_else(|_| panic!("Error connection to {}", database_url))
}
  
#[post("/ingredients/<title>")]
async fn add_ingredient(title: &str, diesel_connection: &State<DieselConnection>) -> Json<Ingredient> {
  use crate::schema::ingredients;

  let conn = &mut *diesel_connection.conn.lock().unwrap();
  let new_ingredient = NewIngredient { title };

  let ingredient = diesel::insert_into(ingredients::table)
    .values(&new_ingredient)
    .get_result::<Ingredient>(conn)
    .expect("New ingredient added");

    Json(ingredient)
}

#[get("/ingredients/<hint>")]
async fn get_ingredients(hint: &str, diesel_connection: &State<DieselConnection>) -> Json<Vec<String>> {
  use self::schema::ingredients::dsl::*;

  let conn = &mut *diesel_connection.conn.lock().unwrap();
  let results = ingredients
    .select(title)
    .limit(5)
    .load::<String>(conn)
    .expect("Error loading ingredient");

  Json(results)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
    .manage(DieselConnection { conn: Arc::new(Mutex::new(connect_db()))})
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
    use crate::Ingredient;
    let client = Client::tracked(rocket()).expect("valid rocket instance");
    let response = client.post(uri!(super::add_ingredient("tomate"))).dispatch();
    let new_ingredient = response.into_json::<Ingredient>().unwrap();
    println!("{:?}", new_ingredient);
    assert_eq!("tomate", new_ingredient.title);
  }
}
