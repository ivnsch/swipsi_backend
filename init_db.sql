-- psql -U ivanschuetz -d bikematch -f ./populate_db.sql

-- reset everything
DROP TABLE bike_pic;
DROP TABLE bike;

-- create tables

CREATE TABLE IF NOT EXISTS bike (
    id SERIAL PRIMARY KEY,
    name_ VARCHAR(255),
    brand VARCHAR(255),
    price VARCHAR(255),
    price_number NUMERIC(10, 2),
    vendor_link VARCHAR(255),
    electric BOOL,
    type_ VARCHAR(255),
    -- todo consider varchar with max limit
    descr TEXT,
    added_timestamp TIMESTAMP DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS bike_pic (
    id SERIAL PRIMARY KEY,
    bike_id INTEGER REFERENCES bike(id),
    -- todo consider varchar with max limit
    url TEXT NOT NULL
);
