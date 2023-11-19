-- Add migration script here

CREATE TABLE env_data(ts INT not null, room TEXT not null, temperature REAL not null, humidity REAL not null);
