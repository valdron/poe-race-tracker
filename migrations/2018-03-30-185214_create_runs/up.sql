-- Your SQL goes here
CREATE TABLE runs (
    id SERIAL PRIMARY KEY,
    duration_in_seconds INTEGER NOT NULL
);

CREATE TABLE levels (
    id SERIAL PRIMARY KEY,
    run_id INTEGER NOT NULL REFERENCES runs (id),
    level SMALLINT NOT NULL,
    duration_in_seconds INTEGER NOT NULL
);

CREATE TABLE zones (
    id SERIAL PRIMARY KEY,
    run_id INTEGER NOT NULL REFERENCES runs (id),
    name VARCHAR NOT NULL,
    duration_in_seconds INTEGER NOT NULL
);