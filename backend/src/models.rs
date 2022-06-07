use diesel::types::Time;
use serde::Deserialize;
use serde::Serialize;
use schema::pets;
use schema::animalShelter;
use schema::appointments;
use schema::citizens;
use schema::fosterHome;
use schema::citizens_appointments;
use schema::employees;
use schema::foundPets;
use schema::holidayCare;
use schema::missingPets;
use schema::petProfiles;
use schema::pets_appointments;
use schema::pets_volunteers;
use schema::volunteers;
use crate::{chrono, DateTime, schema, Utc};

#[derive(Queryable, Identifiable)]
#[table_name ="animalShelter"]
pub struct AnimalShelter {
    pub id: u64,
    pub name: String,
    pub address: String,
    pub capacity: i32
}

#[derive(Deserialize, Insertable)]
#[table_name = "pets"]
pub struct NewPet {
    pub name: String,
    pub species: Option<String>,
    pub description: Option<String>,
    pub volunteer_candidate: bool,
    pub adopted: bool,
    pub holiday_care: bool,
    pub animal_shelter_id: Option<u64>,
    pub foster_home_id: Option<u64>,
    pub adopted_by_cit_id: Option<u64>,
    pub pet_profile_id: Option<u64>,
}

#[derive(Queryable, Identifiable, Associations, PartialEq, Serialize)]
#[belongs_to(AnimalShelter, foreign_key = "animal_shelter_id")]
#[belongs_to(FosterHome, foreign_key = "foster_home_id")]
#[belongs_to(Citizen, foreign_key = "adopted_by_cit_id")]
#[belongs_to(PetProfile, foreign_key = "pet_profile_id")]
#[table_name = "pets"]
pub struct Pet {
    pub id: u64,
    pub name: String,
    pub species: Option<String>,
    pub description: Option<String>,
    pub volunteer_candidate: bool,
    pub adopted: bool,
    pub holiday_care: bool,
    pub animal_shelter_id: Option<u64>,
    pub foster_home_id: Option<u64>,
    pub adopted_by_cit_id: Option<u64>,
    pub pet_profile_id: Option<u64>,
}

#[derive(Queryable, Identifiable, Associations, PartialEq)]
#[belongs_to(Pet, foreign_key = "pet_id")]
#[belongs_to(Appointment, foreign_key = "appointment_id")]
#[table_name = "pets_appointments"]
pub struct PetAppointment {
    pub id: u64,
    pub pet_id: u64,
    pub appointment_id: u64,
}

#[derive(Queryable, Identifiable, Associations, PartialEq)]
#[belongs_to(Pet, foreign_key = "pet_id")]
#[belongs_to(Volunteer, foreign_key = "volunteer_id")]
#[table_name = "pets_volunteers"]
pub struct PetVolunteer {
    pub id: u64,
    pub pet_id: u64,
    pub volunteer_id: u64,
}

#[derive(Queryable, Identifiable, Associations)]
#[table_name = "citizens"]
pub struct Citizen {
    pub id: u64,
    pub name: String,
}

#[derive(Queryable, Identifiable, Associations)]
#[belongs_to(Citizen, foreign_key = "citizen_id")]
#[belongs_to(Appointment, foreign_key = "appointment_id")]
#[table_name = "citizens_appointments"]
pub struct CitizenAppointment {
    pub id: u64,
    pub citizen_id: u64,
    pub appointment_id: u64,
}

#[derive(Queryable, Identifiable, Associations)]
#[table_name = "petProfiles"]
pub struct PetProfile {
    pub id: u64,
    pub picture: String,
    pub profile_text: String,
}

pub enum AppointmentType {
    Course,
    Visit,
    Walk
}

#[derive(Queryable, Identifiable, Associations)]
#[belongs_to(AnimalShelter, foreign_key = "animal_shelter_id")]
#[table_name = "appointments"]
pub struct Appointment {
    pub id: u64,
    pub a_type: AppointmentType,
    pub location: String,
    pub duration_minutes: i32,
    pub description: String,
    pub participants: i32,
    pub animal_shelter_id: u64,
}

#[derive(Queryable, Identifiable, Associations)]
#[table_name = "volunteers"]
pub struct Volunteer {
    pub id: u64,
    pub name: String,
}

#[derive(Queryable, Identifiable, Associations)]
#[belongs_to(AnimalShelter, foreign_key = "animal_shelter_id")]
#[table_name = "employees"]
pub struct Employee {
    pub id: u64,
    pub name: String,
    pub animal_shelter_id: u64,
}

#[derive(Queryable, Identifiable, Associations)]
#[belongs_to(Pet, foreign_key = "pet_id")]
#[table_name = "holidayCare"]
pub struct HolidayCare {
    pub id: u64,
    pub begin: chrono::NaiveDateTime,
    pub end: chrono::NaiveDateTime,
    pub pet_id: u64,
}

#[derive(Queryable, Identifiable, Associations)]
#[belongs_to(AnimalShelter, foreign_key = "animal_shelter_id")]
#[belongs_to(Pet, foreign_key = "pet_id")]
#[table_name = "missingPets"]
pub struct MissingPets {
    pub id: u64,
    pub pet_id: u64,
    pub missing_since: chrono::NaiveDateTime,
    pub owner_id: u64,
    pub location: String,
    pub description: String,
    pub found_on: chrono::NaiveDateTime,
    pub animal_shelter_id: u64,
}

#[derive(Queryable, Identifiable, Associations)]
#[belongs_to(AnimalShelter, foreign_key = "animal_shelter_id")]
#[table_name = "foundPets"]
pub struct FoundPets {
    pub id: u64,
    pub found_on: chrono::NaiveDateTime,
    pub finder_id: u64,
    pub finder_name: String,
    pub description: String,
    pub animal_shelter_id: u64,
}

#[derive(Queryable, Identifiable, Associations)]
#[belongs_to(AnimalShelter, foreign_key = "animal_shelter_id")]
#[table_name = "fosterHome"]
pub struct FosterHome {
    pub id: u64,
    pub capacity: i32,
    pub animal_shelter_id: u64,
}