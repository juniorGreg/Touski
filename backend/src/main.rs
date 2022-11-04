#[macro_use] extern crate rocket;
use rocket::serde::{Serialize, json::Json, Deserialize};
use rocket::State;

use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;

use std::sync::{Arc, Mutex};
use dotenvy::dotenv;
use std::env;

pub mod models;
pub mod schema;

use models::*;
use crate::Ingredient;

fn connect_db() -> Pool<ConnectionManager<PgConnection>> {
  dotenv().ok();
  let database_url= env::var("DATABASE_URL").expect("DATABASE_URL must be set");
  let manager = ConnectionManager::<PgConnection>::new(database_url);
  
  Pool::builder()
    .test_on_check_out(true)
    .build(manager)
    .expect("Could not build connection pool")
}
  
#[post("/ingredients/<title>")]
async fn add_ingredient(title: &str, diesel_connection: &State<Pool<ConnectionManager<PgConnection>>>) -> Json<Ingredient> {
  use crate::schema::ingredients;

  let conn = &mut diesel_connection.get().unwrap();
  let new_ingredient = NewIngredient { title };

  let ingredient = diesel::insert_into(ingredients::table)
    .values(&new_ingredient)
    .get_result::<Ingredient>(conn)
    .expect("New ingredient added");

    Json(ingredient)
}

#[get("/ingredients/<hint>")]
async fn get_ingredients(hint: &str, diesel_connection: &State<Pool<ConnectionManager<PgConnection>>>) -> Json<Vec<String>> {
  use self::schema::ingredients::dsl::*;

  let conn = &mut diesel_connection.get().unwrap();
  let results = ingredients
    .select(title)
    .filter(title.like(format!("{}%", hint)))
    .limit(5)
    .load::<String>(conn)
    .expect("Error loading ingredient");

  Json(results)
}

#[get("/ingredients")]
async fn get_all_ingredients(diesel_connection: &State<Pool<ConnectionManager<PgConnection>>>) -> Json<Vec<String>> {
  use self::schema::ingredients::dsl::*;

  let conn = &mut diesel_connection.get().unwrap();
  let results = ingredients
    .select(title)
    .load::<String>(conn)
    .expect("Error loading ingredient");

  Json(results)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
    .manage(connect_db())
    .mount("/", routes![get_ingredients, get_all_ingredients, add_ingredient])
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
