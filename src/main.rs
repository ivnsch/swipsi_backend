use actix_web::{get, web, App, HttpServer, Responder, Result};
use chrono::Utc;
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct MyObj {
    id: String,
    name: String,
    brand: String,
    price: String,
    price_number: f32,
    pictures: Vec<String>,
    vendor_link: String,
    electric: bool,
    #[serde(rename = "type")]
    type_: BikeType,
    descr: String,
    added_timestamp: u64,
}

#[derive(Serialize)]
enum BikeType {
    Mountain,
    Road,
    Hybrid,
}

#[get("/bikes")]
async fn bikes() -> Result<impl Responder> {
    Ok(web::Json([
        MyObj {
            id: "1".to_string(),
            name: "My Name 1".to_string(),
            brand: "Foo Brand".to_string(),
            price: "999 €".to_string(),
            price_number: 999.,
            pictures: vec![
                "https://picsum.photos/id/0/500/700".to_string(),
                "https://picsum.photos/id/1/500/700".to_string(),
            ],
            vendor_link: "https://google.com".to_string(),
            electric: false,
            type_: BikeType::Mountain,
            descr: "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum".to_string(),
            added_timestamp: Utc::now().timestamp_micros() as u64
        },
        MyObj {
            id: "2".to_string(),
            name: "My Name 2".to_string(),
            brand: "Bar Brand".to_string(),
            price: "2000 €".to_string(),
            price_number: 2000.,
            pictures: vec![
                "https://picsum.photos/id/2/500/700".to_string(),
                "https://picsum.photos/id/3/500/700".to_string(),
            ],
            vendor_link: "https://google.com".to_string(),
            electric: false,
            type_: BikeType::Hybrid,
            descr: "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum".to_string(),
            added_timestamp: Utc::now().timestamp_micros() as u64
        },
        MyObj {
            id: "3".to_string(),
            name: "My Name 3".to_string(),
            brand: "Bar Brand".to_string(),
            price: "580 €".to_string(),
            price_number: 580.,
            pictures: vec![
                "https://picsum.photos/id/4/500/700".to_string(),
                "https://picsum.photos/id/5/500/700".to_string(),
            ],
            vendor_link: "https://google.com".to_string(),
            electric: true,
            type_: BikeType::Road,
            descr: "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum".to_string(),
            added_timestamp: Utc::now().timestamp_micros() as u64
        },
    ]))
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(bikes))
        // .bind(("127.0.0.1", 8080))?
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
