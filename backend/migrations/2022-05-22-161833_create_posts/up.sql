-- Your SQL goes here

CREATE TABLE animalShelter (
    id SERIAL PRIMARY KEY,
    name VARCHAR(150),
    address VARCHAR(200),
    capacity INTEGER
);

CREATE TABLE citizens (
    id SERIAL PRIMARY KEY,
    name VARCHAR(150)
);

CREATE TABLE fosterHome (
    id SERIAL PRIMARY KEY,
    capacity INTEGER,
    animal_shelter_id BIGINT UNSIGNED,
    FOREIGN KEY (animal_shelter_id) REFERENCES animalShelter(id)
);

CREATE TABLE petProfiles (
    id SERIAL PRIMARY KEY,
    picture VARCHAR(150),
    profile_text VARCHAR(10000)
);

CREATE TABLE pets (
    id SERIAL PRIMARY KEY,
    name VARCHAR(30) NOT NULL,
    species VARCHAR(150),
    description VARCHAR(1000),
    volunteer_candidate BOOLEAN NOT NULL DEFAULT FALSE,
    adopted BOOLEAN,
    holiday_care BOOLEAN,
    animal_shelter_id BIGINT UNSIGNED,
    foster_home_id BIGINT UNSIGNED,
    adopted_by_cit_id BIGINT UNSIGNED,
    pet_profile_id BIGINT UNSIGNED,
    FOREIGN KEY (animal_shelter_id) REFERENCES animalShelter(id),
    FOREIGN KEY (foster_home_id) REFERENCES  fosterHome(id),
    FOREIGN KEY (adopted_by_cit_id) REFERENCES citizens(id),
    FOREIGN KEY (pet_profile_id) REFERENCES petProfiles(id)
);

CREATE TABLE appointments (
    id SERIAL PRIMARY KEY,
    a_type VARCHAR(30),
    location VARCHAR(200),
    duration_minutes INTEGER,
    description VARCHAR(1000),
    participants INTEGER,
    animal_shelter_id BIGINT UNSIGNED,
    FOREIGN KEY (animal_shelter_id) REFERENCES animalShelter(id)
);

CREATE TABLE pets_appointments (
    id SERIAL PRIMARY KEY,
    pet_id BIGINT UNSIGNED,
    appointment_id BIGINT UNSIGNED,
    FOREIGN KEY (pet_id) REFERENCES pets(id),
    FOREIGN KEY (appointment_id) REFERENCES appointments(id)
);

CREATE TABLE volunteers (
    id SERIAL PRIMARY KEY,
    name VARCHAR(150)
);

CREATE TABLE pets_volunteers (
    id SERIAL PRIMARY KEY,
    pet_id BIGINT UNSIGNED,
    volunteer_id BIGINT UNSIGNED,
    FOREIGN KEY (pet_id) REFERENCES pets(id),
    FOREIGN KEY (volunteer_id) REFERENCES volunteers(id)
);

CREATE TABLE citizens_appointments (
    id SERIAL PRIMARY KEY,
    citizen_id BIGINT UNSIGNED,
    appointment_id BIGINT UNSIGNED,
    FOREIGN KEY (citizen_id) REFERENCES citizens(id),
    FOREIGN KEY (appointment_id) REFERENCES appointments(id)
);

CREATE TABLE employees (
    id SERIAL PRIMARY KEY,
    name VARCHAR(150),
    animal_shelter_id BIGINT UNSIGNED,
    FOREIGN KEY (animal_shelter_id) REFERENCES  animalShelter(id)
);

CREATE TABLE holidayCare (
    id SERIAL PRIMARY KEY,
    begin DATETIME,
    end DATETIME,
    pet_id BIGINT UNSIGNED,
    FOREIGN KEY (pet_id) REFERENCES pets(id)
);

CREATE TABLE missingPets (
    id SERIAL PRIMARY KEY,
    pet_id BIGINT UNSIGNED,
    missing_since DATETIME,
    owner_id INTEGER,
    location VARCHAR(200),
    description VARCHAR(1000),
    found_on DATETIME,
    animal_shelter_id BIGINT UNSIGNED,
    FOREIGN KEY (animal_shelter_id) REFERENCES animalShelter(id),
    FOREIGN KEY (pet_id) REFERENCES pets(id)
);

CREATE TABLE foundPets (
     id SERIAL PRIMARY KEY,
     pet_id INTEGER,
     found_on DATETIME,
     finder_id INTEGER,
     finder_name VARCHAR(150),
     description VARCHAR(1000),
     animal_shelter_id BIGINT UNSIGNED,
     FOREIGN KEY (animal_shelter_id) REFERENCES animalShelter(id)
);


CREATE TABLE animalShelter (
    id SERIAL PRIMARY KEY,
    name VARCHAR(150),
    address VARCHAR(200),
    capacity INTEGER
);

CREATE TABLE citizens (
    id SERIAL PRIMARY KEY,
    name VARCHAR(150)
);

CREATE TABLE fosterHome (
    id SERIAL PRIMARY KEY,
    capacity INTEGER,
    animal_shelter_id BIGINT UNSIGNED,
    FOREIGN KEY (animal_shelter_id) REFERENCES animalShelter(id)
);

CREATE TABLE petProfiles (
    id SERIAL PRIMARY KEY,
    picture VARCHAR(150),
    profile_text VARCHAR(10000)
);

CREATE TABLE pets (
    id SERIAL PRIMARY KEY,
    name VARCHAR(30) NOT NULL,
    species VARCHAR(150),
    description VARCHAR(1000),
    volunteer_candidate BOOLEAN NOT NULL DEFAULT FALSE,
    adopted BOOLEAN,
    holiday_care BOOLEAN,
    animal_shelter_id BIGINT UNSIGNED,
    foster_home_id BIGINT UNSIGNED,
    adopted_by_cit_id BIGINT UNSIGNED,
    pet_profile_id BIGINT UNSIGNED,
    FOREIGN KEY (animal_shelter_id) REFERENCES animalShelter(id),
    FOREIGN KEY (foster_home_id) REFERENCES  fosterHome(id),
    FOREIGN KEY (adopted_by_cit_id) REFERENCES citizens(id),
    FOREIGN KEY (pet_profile_id) REFERENCES petProfiles(id)
);

CREATE TABLE appointments (
    id SERIAL PRIMARY KEY,
    a_type VARCHAR(30),
    location VARCHAR(200),
    duration_minutes INTEGER,
    description VARCHAR(1000),
    participants INTEGER,
    animal_shelter_id BIGINT UNSIGNED,
    FOREIGN KEY (animal_shelter_id) REFERENCES animalShelter(id)
);

CREATE TABLE pets_appointments (
    id SERIAL PRIMARY KEY,
    pet_id BIGINT UNSIGNED,
    appointment_id BIGINT UNSIGNED,
    FOREIGN KEY (pet_id) REFERENCES pets(id),
    FOREIGN KEY (appointment_id) REFERENCES appointments(id)
);

CREATE TABLE volunteers (
    id SERIAL PRIMARY KEY,
    name VARCHAR(150)
);

CREATE TABLE pets_volunteers (
    id SERIAL PRIMARY KEY,
    pet_id BIGINT UNSIGNED,
    volunteer_id BIGINT UNSIGNED,
    FOREIGN KEY (pet_id) REFERENCES pets(id),
    FOREIGN KEY (volunteer_id) REFERENCES volunteers(id)
);

CREATE TABLE citizens_appointments (
    id SERIAL PRIMARY KEY,
    citizen_id BIGINT UNSIGNED,
    appointment_id BIGINT UNSIGNED,
    FOREIGN KEY (citizen_id) REFERENCES citizens(id),
    FOREIGN KEY (appointment_id) REFERENCES appointments(id)
);

CREATE TABLE employees (
    id SERIAL PRIMARY KEY,
    name VARCHAR(150),
    animal_shelter_id BIGINT UNSIGNED,
    FOREIGN KEY (animal_shelter_id) REFERENCES  animalShelter(id)
);

CREATE TABLE holidayCare (
    id SERIAL PRIMARY KEY,
    begin DATETIME,
    end DATETIME,
    pet_id BIGINT UNSIGNED,
    FOREIGN KEY (pet_id) REFERENCES pets(id)
);

CREATE TABLE missingPets (
    id SERIAL PRIMARY KEY,
    pet_id BIGINT UNSIGNED,
    missing_since DATETIME,
    owner_id INTEGER,
    location VARCHAR(200),
    description VARCHAR(1000),
    found_on DATETIME,
    animal_shelter_id BIGINT UNSIGNED,
    FOREIGN KEY (animal_shelter_id) REFERENCES animalShelter(id),
    FOREIGN KEY (pet_id) REFERENCES pets(id)
);

CREATE TABLE foundPets (
     id SERIAL PRIMARY KEY,
     pet_id INTEGER,
     found_on DATETIME,
     finder_id INTEGER,
     finder_name VARCHAR(150),
     description VARCHAR(1000),
     animal_shelter_id BIGINT UNSIGNED,
     FOREIGN KEY (animal_shelter_id) REFERENCES animalShelter(id)
);


CREATE TABLE animalShelter (
    id SERIAL PRIMARY KEY,
    name VARCHAR(150),
    address VARCHAR(200),
    capacity INTEGER
);

CREATE TABLE citizens (
    id SERIAL PRIMARY KEY,
    name VARCHAR(150)
);

CREATE TABLE fosterHome (
    id SERIAL PRIMARY KEY,
    capacity INTEGER,
    animal_shelter_id BIGINT UNSIGNED,
    FOREIGN KEY (animal_shelter_id) REFERENCES animalShelter(id)
);

CREATE TABLE petProfiles (
    id SERIAL PRIMARY KEY,
    picture VARCHAR(150),
    profile_text VARCHAR(10000)
);

CREATE TABLE pets (
    id SERIAL PRIMARY KEY,
    name VARCHAR(30) NOT NULL,
    species VARCHAR(150),
    description VARCHAR(1000),
    volunteer_candidate BOOLEAN NOT NULL DEFAULT FALSE,
    adopted BOOLEAN,
    holiday_care BOOLEAN,
    animal_shelter_id BIGINT UNSIGNED,
    foster_home_id BIGINT UNSIGNED,
    adopted_by_cit_id BIGINT UNSIGNED,
    pet_profile_id BIGINT UNSIGNED,
    FOREIGN KEY (animal_shelter_id) REFERENCES animalShelter(id),
    FOREIGN KEY (foster_home_id) REFERENCES  fosterHome(id),
    FOREIGN KEY (adopted_by_cit_id) REFERENCES citizens(id),
    FOREIGN KEY (pet_profile_id) REFERENCES petProfiles(id)
);

CREATE TABLE appointments (
    id SERIAL PRIMARY KEY,
    a_type VARCHAR(30),
    location VARCHAR(200),
    duration_minutes INTEGER,
    description VARCHAR(1000),
    participants INTEGER,
    animal_shelter_id BIGINT UNSIGNED,
    FOREIGN KEY (animal_shelter_id) REFERENCES animalShelter(id)
);

CREATE TABLE pets_appointments (
    id SERIAL PRIMARY KEY,
    pet_id BIGINT UNSIGNED,
    appointment_id BIGINT UNSIGNED,
    FOREIGN KEY (pet_id) REFERENCES pets(id),
    FOREIGN KEY (appointment_id) REFERENCES appointments(id)
);

CREATE TABLE volunteers (
    id SERIAL PRIMARY KEY,
    name VARCHAR(150)
);

CREATE TABLE pets_volunteers (
    id SERIAL PRIMARY KEY,
    pet_id BIGINT UNSIGNED,
    volunteer_id BIGINT UNSIGNED,
    FOREIGN KEY (pet_id) REFERENCES pets(id),
    FOREIGN KEY (volunteer_id) REFERENCES volunteers(id)
);

CREATE TABLE citizens_appointments (
    id SERIAL PRIMARY KEY,
    citizen_id BIGINT UNSIGNED,
    appointment_id BIGINT UNSIGNED,
    FOREIGN KEY (citizen_id) REFERENCES citizens(id),
    FOREIGN KEY (appointment_id) REFERENCES appointments(id)
);

CREATE TABLE employees (
    id SERIAL PRIMARY KEY,
    name VARCHAR(150),
    animal_shelter_id BIGINT UNSIGNED,
    FOREIGN KEY (animal_shelter_id) REFERENCES  animalShelter(id)
);

CREATE TABLE holidayCare (
    id SERIAL PRIMARY KEY,
    begin DATETIME,
    end DATETIME,
    pet_id BIGINT UNSIGNED,
    FOREIGN KEY (pet_id) REFERENCES pets(id)
);

CREATE TABLE missingPets (
    id SERIAL PRIMARY KEY,
    pet_id BIGINT UNSIGNED,
    missing_since DATETIME,
    owner_id INTEGER,
    location VARCHAR(200),
    description VARCHAR(1000),
    found_on DATETIME,
    animal_shelter_id BIGINT UNSIGNED,
    FOREIGN KEY (animal_shelter_id) REFERENCES animalShelter(id),
    FOREIGN KEY (pet_id) REFERENCES pets(id)
);

CREATE TABLE foundPets (
     id SERIAL PRIMARY KEY,
     pet_id INTEGER,
     found_on DATETIME,
     finder_id INTEGER,
     finder_name VARCHAR(150),
     description VARCHAR(1000),
     animal_shelter_id BIGINT UNSIGNED,
     FOREIGN KEY (animal_shelter_id) REFERENCES animalShelter(id)
);

