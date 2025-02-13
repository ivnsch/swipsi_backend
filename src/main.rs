pub mod scrapper;

use actix_web::{
    get,
    middleware::Logger,
    post,
    web::{self, Data},
    App, HttpServer, Responder, Result,
};
use log::info;
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPoolOptions, prelude::FromRow, Pool, Postgres};

#[derive(Debug, FromRow, Serialize)]
#[serde(rename_all = "camelCase")]
struct Bike {
    id: String,
    #[serde(rename = "name")]
    name_: String,
    price: String,
    price_number: f32,
    price_currency: String,
    pictures: Vec<String>,
    vendor_link: String,
    type_: String,
    descr: String,
    added_timestamp: i64,
}

#[derive(Debug, Deserialize)]
struct Filters {
    type_: Vec<String>,
    price: Vec<u32>,
}

#[derive(Debug)]
struct DbFilters {
    type_: Vec<String>,
    price_min: f32,
    price_max: f32,
}

#[post("/items/{last_timestamp}")]
async fn items(
    state: Data<AppState>,
    path: web::Path<i64>,
    filters: web::Json<Filters>,
) -> Result<impl Responder> {
    let last_timestamp = path.into_inner();
    Ok(web::Json(
        load_items(&state.db, last_timestamp, &to_db_filters(&filters)).await,
    ))
}

fn to_min_max(price_filter: &[u32]) -> PriceBounds {
    let min_possible_price = 0.;
    let max_possible_price = 1_000_000.;

    if price_filter.is_empty() {
        return PriceBounds {
            min: min_possible_price,
            max: max_possible_price,
        };
    }

    let mut min = f32::MAX;
    let mut max: f32 = 0.;

    if price_filter.contains(&1) {
        min = min.min(min_possible_price);
        max = max.max(19.99);
    }
    if price_filter.contains(&2) {
        min = min.min(20.);
        max = max.max(49.99);
    }
    if price_filter.contains(&3) {
        min = min.min(50.);
        max = max.max(99.99);
    }
    if price_filter.contains(&4) {
        min = min.min(100.);
        max = max.max(1_000_000.);
    }

    PriceBounds { min, max }
}

struct PriceBounds {
    min: f32,
    max: f32,
}

fn to_db_filters(filters: &Filters) -> DbFilters {
    let type_filter = if filters.type_.is_empty() {
        vec![
            "necklace".to_string(),
            "bracelet".to_string(),
            "ring".to_string(),
            "earring".to_string(),
        ]
    } else {
        filters.type_.clone()
    };

    let price_bounds = to_min_max(&filters.price);

    DbFilters {
        type_: type_filter,
        price_min: price_bounds.min,
        price_max: price_bounds.max,
    }
}

// TODO redunancy filters price-filters
async fn load_items(pool: &Pool<Postgres>, after_timestamp: i64, filters: &DbFilters) -> Vec<Bike> {
    info!("filters: {:?}", filters);

    let res: Vec<Bike> = sqlx::query_as(
        r#"
SELECT
    b.id::TEXT AS id,
    b.name_,
    b.price,
    b.price_number,
    b.price_currency,
    b.vendor_link,
    b.type_,
    b.descr,
    b.added_timestamp,
    COALESCE(array_agg(bp.url) FILTER (WHERE bp.url IS NOT NULL), ARRAY[]::TEXT[]) AS pictures
FROM
    bike b
LEFT JOIN
    bike_pic bp ON b.id = bp.bike_id
WHERE
    b.added_timestamp > $1 AND b.type_ = ANY($2) AND b.price_number > $3 AND b.price_number < $4
GROUP BY
    b.id, b.name_, b.price, b.price_number, b.price_currency, b.vendor_link, b.type_, b.descr, b.added_timestamp
LIMIT 50;
"#,
    )
    .bind(after_timestamp.clone())
    .bind(filters.type_.clone())
    .bind(filters.price_min)
    .bind(filters.price_max)
    .fetch_all(pool)
    .await
    .expect("error2");

    res
}

pub struct AppState {
    db: Pool<Postgres>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));

    let pool = init_pool().await;

    // consider removing this..
    test_query(&pool).await;

    pool.acquire().await.expect("error acquiring connection");

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(Data::new(AppState { db: pool.clone() }))
            .service(items)
    })
    // .bind(("127.0.0.1", 8080))?
    .bind(("0.0.0.0", 3000))?
    .run()
    .await
}

async fn test_query(pool: &Pool<Postgres>) {
    // Make a simple query to return the given parameter (use a question mark `?` instead of `$1` for MySQL/MariaDB)
    let row: (i64,) = sqlx::query_as("SELECT $1")
        .bind(150_i64)
        .fetch_one(pool)
        .await
        .expect("error2");
    println!("row: {row:?}");
    assert_eq!(row.0, 150);
}

async fn init_pool() -> Pool<Postgres> {
    PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://tester:testpw@localhost:5433/bikematch")
        .await
        .expect("error1")
}

#[cfg(test)]
mod test {
    use crate::{init_pool, load_items, to_db_filters, Filters};

    #[tokio::test]
    async fn test_load_items_after_timestamp() {
        let pool = init_pool().await;

        let filters = Filters {
            type_: vec!["necklace".to_string(), "bracelet".to_string()],
            price: vec![1, 2, 3, 4],
        };

        let items = load_items(&pool, 0, &to_db_filters(&filters)).await;
        println!("loaded all items len: {}", items.len());

        let items = load_items(&pool, 1739368334742824, &to_db_filters(&filters)).await;
        println!("loaded  items after timestamp len: {}", items.len());
    }
}
