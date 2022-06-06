table! {
    animalShelter (id) {
        id -> Unsigned<Bigint>,
        name -> Nullable<Varchar>,
        address -> Nullable<Varchar>,
        capacity -> Nullable<Integer>,
    }
}

table! {
    appointments (id) {
        id -> Unsigned<Bigint>,
        a_type -> Nullable<Varchar>,
        location -> Nullable<Varchar>,
        duration_minutes -> Nullable<Integer>,
        description -> Nullable<Varchar>,
        participants -> Nullable<Integer>,
        animal_shelter_id -> Nullable<Unsigned<Bigint>>,
    }
}

table! {
    citizens (id) {
        id -> Unsigned<Bigint>,
        name -> Nullable<Varchar>,
    }
}

table! {
    citizens_appointments (id) {
        id -> Unsigned<Bigint>,
        citizen_id -> Nullable<Unsigned<Bigint>>,
        appointment_id -> Nullable<Unsigned<Bigint>>,
    }
}

table! {
    employees (id) {
        id -> Unsigned<Bigint>,
        name -> Nullable<Varchar>,
        animal_shelter_id -> Nullable<Unsigned<Bigint>>,
    }
}

table! {
    fosterHome (id) {
        id -> Unsigned<Bigint>,
        capacity -> Nullable<Integer>,
        animal_shelter_id -> Nullable<Unsigned<Bigint>>,
    }
}

table! {
    foundPets (id) {
        id -> Unsigned<Bigint>,
        pet_id -> Nullable<Integer>,
        found_on -> Nullable<Datetime>,
        finder_id -> Nullable<Integer>,
        finder_name -> Nullable<Varchar>,
        description -> Nullable<Varchar>,
        animal_shelter_id -> Nullable<Unsigned<Bigint>>,
    }
}

table! {
    holidayCare (id) {
        id -> Unsigned<Bigint>,
        begin -> Nullable<Datetime>,
        end -> Nullable<Datetime>,
        pet_id -> Nullable<Unsigned<Bigint>>,
    }
}

table! {
    missingPets (id) {
        id -> Unsigned<Bigint>,
        pet_id -> Nullable<Unsigned<Bigint>>,
        missing_since -> Nullable<Datetime>,
        owner_id -> Nullable<Integer>,
        location -> Nullable<Varchar>,
        description -> Nullable<Varchar>,
        found_on -> Nullable<Datetime>,
        animal_shelter_id -> Nullable<Unsigned<Bigint>>,
    }
}

table! {
    petProfiles (id) {
        id -> Unsigned<Bigint>,
        picture -> Nullable<Varchar>,
        profile_text -> Nullable<Varchar>,
    }
}

table! {
    pets (id) {
        id -> Unsigned<Bigint>,
        name -> Varchar,
        species -> Nullable<Varchar>,
        description -> Nullable<Varchar>,
        volunteer_candidate -> Bool,
        adopted -> Nullable<Bool>,
        holiday_care -> Nullable<Bool>,
        animal_shelter_id -> Nullable<Unsigned<Bigint>>,
        foster_home_id -> Nullable<Unsigned<Bigint>>,
        adopted_by_cit_id -> Nullable<Unsigned<Bigint>>,
        pet_profile_id -> Nullable<Unsigned<Bigint>>,
    }
}

table! {
    pets_appointments (id) {
        id -> Unsigned<Bigint>,
        pet_id -> Nullable<Unsigned<Bigint>>,
        appointment_id -> Nullable<Unsigned<Bigint>>,
    }
}

table! {
    pets_volunteers (id) {
        id -> Unsigned<Bigint>,
        pet_id -> Nullable<Unsigned<Bigint>>,
        volunteer_id -> Nullable<Unsigned<Bigint>>,
    }
}

table! {
    volunteers (id) {
        id -> Unsigned<Bigint>,
        name -> Nullable<Varchar>,
    }
}

joinable!(appointments -> animalShelter (animal_shelter_id));
joinable!(citizens_appointments -> appointments (appointment_id));
joinable!(citizens_appointments -> citizens (citizen_id));
joinable!(employees -> animalShelter (animal_shelter_id));
joinable!(fosterHome -> animalShelter (animal_shelter_id));
joinable!(foundPets -> animalShelter (animal_shelter_id));
joinable!(holidayCare -> pets (pet_id));
joinable!(missingPets -> animalShelter (animal_shelter_id));
joinable!(missingPets -> pets (pet_id));
joinable!(pets -> animalShelter (animal_shelter_id));
joinable!(pets -> citizens (adopted_by_cit_id));
joinable!(pets -> fosterHome (foster_home_id));
joinable!(pets -> petProfiles (pet_profile_id));
joinable!(pets_appointments -> appointments (appointment_id));
joinable!(pets_appointments -> pets (pet_id));
joinable!(pets_volunteers -> pets (pet_id));
joinable!(pets_volunteers -> volunteers (volunteer_id));

allow_tables_to_appear_in_same_query!(
    animalShelter,
    appointments,
    citizens,
    citizens_appointments,
    employees,
    fosterHome,
    foundPets,
    holidayCare,
    missingPets,
    petProfiles,
    pets,
    pets_appointments,
    pets_volunteers,
    volunteers,
);
