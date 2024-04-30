CREATE TABLE googleusers (
                             id  serial NOT NULL PRIMARY KEY ,
                             sub TEXT UNIQUE NOT NULL,
                             picture TEXT,
                             email TEXT UNIQUE NOT NULL,
                             name TEXT UNIQUE NOT NULL,
                             token TEXT UNIQUE,
                             phone_number TEXT
--     calendar TEXT UNIQUE NOT NULL
);

CREATE TABLE MX (
                    id serial NOT NULL PRIMARY KEY ,
                    mx_index INTEGER NOT NULL,
                    owner INTEGER  NOT NULL,
                    date DATE UNIQUE NOT NULL,
                    title TEXT UNIQUE NOT NULL,
                    description TEXT UNIQUE NOT NULL
--     FOREIGN KEY(owner) REFERENCES user(id)
);
-- Add up migration script here
