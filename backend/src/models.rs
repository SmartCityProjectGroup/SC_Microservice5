use diesel::types::Time;

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

pub enum Appointment_type {
    course,
    visit,
    walk
}

#[derive(Queryable)]
pub struct Appointment {
    pub id: i32,
    pub a_type: Appointment_type,
    pub location: String,
    pub durationMinutes: i32,
    pub description: String,
    pub participants: i32,
}