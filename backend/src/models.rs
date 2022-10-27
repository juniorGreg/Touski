use diesel::prelude::*;
use rocket::serde::{Serialize, Deserialize};
use crate::schema::ingredients;

#[derive(Queryable, Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Ingredient {
  pub id: i64,
  pub title: String
}

#[derive(Insertable)]
#[diesel(table_name = ingredients)]
pub struct NewIngredient<'a> {
  pub title: &'a str
}
