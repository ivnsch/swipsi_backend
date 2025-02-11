-- psql -U ivanschuetz -d bikematch -f ./init_db.sql

-- reset everything
DROP TABLE bike_pic;
DROP TABLE bike;

-- create tables

CREATE TABLE IF NOT EXISTS bike (
    id SERIAL PRIMARY KEY,
    name_ VARCHAR(255),
    brand VARCHAR(255),
    price VARCHAR(255),
    price_number FLOAT4,
    vendor_link VARCHAR(255),
    electric BOOL,
    type_ VARCHAR(255),
    -- todo consider varchar with max limit
    descr TEXT,
    added_timestamp BIGINT
);

CREATE TABLE IF NOT EXISTS bike_pic (
    id SERIAL PRIMARY KEY,
    bike_id INTEGER REFERENCES bike(id),
    -- todo consider varchar with max limit
    url TEXT NOT NULL
);
