-- psql -p 5433 -U ivanschuetz -d bikematch -f ./init_db.sql

-- reset everything
DROP TABLE if exists item_pic;
DROP TABLE if exists item;

-- create tables

CREATE TABLE IF NOT EXISTS item (
    id SERIAL PRIMARY KEY,
    name_ VARCHAR(255),
    price VARCHAR(255),
    price_number FLOAT4,
    price_currency VARCHAR(255),
    vendor_link VARCHAR(255),
    type_ VARCHAR(255),
    -- todo consider varchar with max limit
    descr TEXT,
    added_timestamp BIGINT
);

CREATE TABLE IF NOT EXISTS item_pic (
    id SERIAL PRIMARY KEY,
    item_id INTEGER REFERENCES item(id),
    -- todo consider varchar with max limit
    url TEXT NOT NULL
);
