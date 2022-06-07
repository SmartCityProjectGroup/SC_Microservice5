use crate::{router::Route};
use zoon::{eprintln, routing::origin, *};

// ------ ------
//     Types
// ------ ------

#[derive(Clone, Copy, PartialEq, Eq)]
enum Pet_List {
    Loading,
    Loaded(u32),
    Failed
}

// ------ ------
//    States
// ------ ------

#[static_ref]
fn pet_list() -> &'static Mutable<Pet_List>{
    Mutable::new(Pet_List::Loading)
}

// ------ ------
//   Commands
// ------ ------

pub fn load_pet_list() -> impl Element{
    async fn pet_list_request() -> reqwest::Result<u32> {
        Ok(reqwest::get(origin() + "/showShelterPets")
            .await?
            .error_for_status()?
            .text()
            .await?
            .parse()
            .unwrap_throw())
    }
    Task::start(async {
        pet_list().set_neq(Pet_List::Loading);
        match pet_list_request().await {
            Ok(loaded_pet_list) => pet_list().set_neq(Pet_List::Loaded(loaded_pet_list)),
            Err(error) => {
                eprintln!("pet_list request failed: {:#?}", error);
                pet_list().set_neq(Pet_List::Failed)
            }
        }
    });
    El::new().child("Pets")
}

// ------ ------
//     View
// ------ ------

pub fn page() -> impl Element {
    Column::new()
        .item(label())
        .item(pet_list_view())
}

fn label() -> impl Element {
    Row::new()
        .item(El::new().child("Tiere im Tierheim"))
}

fn pet_list_view() -> impl Element {
    Text::with_signal(pet_list().signal().map(|pet_list| match pet_list {
        Pet_List::Loading => "Loading...".into_cow_str(),
        Pet_List::Loaded(pet_list) => pet_list.into_cow_str(),
        Pet_List::Failed => "Loading failed!".into_cow_str(),
    }))
}




