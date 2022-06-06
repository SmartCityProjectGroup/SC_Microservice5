extern crate diesel;
extern crate dotenv;

use diesel::{BoolExpressionMethods, insert_into, MysqlConnection, QueryDsl, RunQueryDsl};
use serde_json::Value::Null;
use crate::models::{NewPet, Pet};
use crate::schema::pets::adopted;
use crate::schema::pets::dsl::pets;
use crate::web::HttpResponse;
use crate::diesel::ExpressionMethods;

pub fn insert_new_pet(conn: &MysqlConnection, pet: NewPet) {
    insert_into(pets)
        .values(&pet)
        .execute(conn)
        .unwrap();
}

pub fn select_shelter_pets(conn: &MysqlConnection) -> Vec<Pet> {
     pets
            .filter(adopted.eq(false))
            .load::<Pet>(conn)
            .expect("Error loading pets")
}