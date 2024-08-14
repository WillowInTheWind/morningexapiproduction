
CREATE TABLE googleusers (
    id  serial NOT NULL PRIMARY KEY ,
     sub TEXT UNIQUE NOT NULL,
     picture TEXT,
     email TEXT UNIQUE NOT NULL,
    name TEXT UNIQUE NOT NULL,
    token TEXT UNIQUE,
    phone_number TEXT
    is_admin BOOLEAN NOT NULL
--     calendar TEXT UNIQUE NOT NULL
);

CREATE TABLE MX (
     id serial NOT NULL PRIMARY KEY ,
     owner INTEGER  NOT NULL,
     date DATE UNIQUE NOT NULL,
     title TEXT UNIQUE NOT NULL,
     description TEXT UNIQUE NOT NULL,
     -- new shit
     min_grade INTEGER NOT NULL,
     max_grade INTEGER NOT NULL,
     young_student_prep_instructions TEXT NOT NULL,
     is_available_in_day BOOLEAN NOT NULL,
     required_tech_json TEXT NOT NULL,
     short_description TEXT NOT NULL,
     editors_json TEXT NOT NULL,
     is_approved BOOLEAN NOT NULL
--     FOREIGN KEY(owner) REFERENCES user(id)
);
-- Add up migration script here
