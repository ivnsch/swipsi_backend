pub mod scrapper;

use actix_web::{
    get,
    middleware::Logger,
    web::{self, Data},
    App, HttpServer, Responder, Result,
};
use serde::Serialize;
use sqlx::{postgres::PgPoolOptions, prelude::FromRow, Pool, Postgres};

#[derive(Debug, FromRow, Serialize)]
#[serde(rename_all = "camelCase")]
struct Bike {
    id: String,
    #[serde(rename = "name")]
    name_: String,
    price: String,
    price_number: f32,
    pictures: Vec<String>,
    vendor_link: String,
    type_: String,
    descr: String,
    added_timestamp: i64,
}

#[get("/bikes")]
async fn bikes(state: Data<AppState>) -> Result<impl Responder> {
    Ok(web::Json(load_bikes(&state.db).await))
}

#[get("/items/{last_timestamp}")]
async fn items(state: Data<AppState>, path: web::Path<i64>) -> Result<impl Responder> {
    let last_timestamp = path.into_inner();
    Ok(web::Json(load_items(&state.db, last_timestamp).await))
}

async fn load_bikes(pool: &Pool<Postgres>) -> Vec<Bike> {
    let res: Vec<Bike> = sqlx::query_as(
        r#"
SELECT
    b.id::TEXT AS id,
    b.name_,
    b.price,
    b.price_number,
    b.vendor_link,
    b.type_,
    b.descr,
    b.added_timestamp,
    COALESCE(array_agg(bp.url) FILTER (WHERE bp.url IS NOT NULL), ARRAY[]::TEXT[]) AS pictures
FROM
    bike b
LEFT JOIN
    bike_pic bp ON b.id = bp.bike_id
GROUP BY
    b.id, b.name_, b.price, b.price_number, b.vendor_link, b.type_, b.descr, b.added_timestamp;
"#,
    )
    .fetch_all(pool)
    .await
    .expect("error2");

    res
}

async fn load_items(pool: &Pool<Postgres>, after_timestamp: i64) -> Vec<Bike> {
    let res: Vec<Bike> = sqlx::query_as(
        r#"
SELECT
    b.id::TEXT AS id,
    b.name_,
    b.price,
    b.price_number,
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
    b.added_timestamp > $1
GROUP BY
    b.id, b.name_, b.price, b.price_number, b.vendor_link, b.type_, b.descr, b.added_timestamp
LIMIT 50;
"#,
    )
    .bind(after_timestamp.clone())
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
            .service(bikes)
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
    use crate::{init_pool, load_bikes, load_items};

    #[tokio::test]
    async fn test_load_bikes() {
        let pool = init_pool().await;
        let bikes = load_bikes(&pool).await;
        println!("bikes: {:?}", bikes);
    }

    #[tokio::test]
    async fn test_load_items_after_timestamp() {
        let pool = init_pool().await;
        let items = load_items(&pool, 0).await;
        println!("loaded all items len: {}", items.len());

        let items = load_items(&pool, 1739368334742824).await;
        println!("loaded  items after timestamp len: {}", items.len());
    }
}
