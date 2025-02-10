-- psql -U ivanschuetz -d bikematch -f ./populate_db.sql

-- uncomment to reset everything
-- DROP TABLE bike;
-- DROP TABLE bike_pic;

CREATE TABLE IF NOT EXISTS bike (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255),
    brand VARCHAR(255),
    price VARCHAR(255),
    price_number VARCHAR(255),
    vendor_link VARCHAR(255),
    electric BOOL,
    type_ VARCHAR(255),
    descr VARCHAR(255),
    added_timestamp TIMESTAMP
);

CREATE TABLE IF NOT EXISTS bike_pic (
    id SERIAL PRIMARY KEY,
    bike_id INTEGER REFERENCES bike(id),
    url TEXT NOT NULL
);
