#[derive(Queryable)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}
#[derive(Queryable)]
pub struct Pet {
    pub id: i32,
    pub name: String,
    pub species: String,
    pub description: String,
    pub volunteer_candidate: bool,
}

#[derive(Queryable)]
pub struct Citizen {
    pub id: i32,
    pub name: String,
}

#[derive(Queryable)]
pub struct PetProfile {
    pub id: i32,
    pub picture: String,
    pub profiletext: String,
}