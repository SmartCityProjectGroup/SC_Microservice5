use moon::actix_web::{HttpResponse, web};
use crate::actions::{insert_new_pet, select_shelter_pets};
use crate::connect::DBPool;
use crate::models::NewPet;

pub async fn add_new_pet(pool: web::Data<DBPool>, pet: web::Json<NewPet>) -> HttpResponse {
    let new_pet = pet.into_inner();
    let db = pool.get().unwrap();

    web::block(move || insert_new_pet(&db, new_pet))
        .await
        .unwrap();
    HttpResponse::Ok().finish()
}

pub async fn show_shelter_pets(pool: web::Data<DBPool>) -> HttpResponse {
    let db = pool.get().unwrap();
    let pets = select_shelter_pets(&db);
    HttpResponse::Ok().json(pets)
}