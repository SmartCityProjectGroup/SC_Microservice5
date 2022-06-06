extern crate diesel;
extern crate dotenv;

use diesel::{insert_into, MysqlConnection, RunQueryDsl};
use crate::models::NewPet;
use crate::schema::pets::dsl::pets;


pub fn insert_new_pet(conn: &MysqlConnection, pet: NewPet) {
    insert_into(pets)
        .values(&pet)
        .execute(conn)
        .unwrap();
}