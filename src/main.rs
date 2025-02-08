use actix_web::{get, web, App, HttpServer, Responder, Result};
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct MyObj {
    id: String,
    name: String,
    brand: String,
    price: String,
    pictures: Vec<String>,
    vendor_link: String,
    electric: bool,
    #[serde(rename = "type")]
    type_: BikeType,
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
            price: "999".to_string(),
            pictures: vec![
                "https://picsum.photos/id/0/500/700".to_string(),
                "https://picsum.photos/id/1/500/700".to_string(),
            ],
            vendor_link: "https://google.com".to_string(),
            electric: false,
            type_: BikeType::Mountain,
        },
        MyObj {
            id: "2".to_string(),
            name: "My Name 2".to_string(),
            brand: "Bar Brand".to_string(),
            price: "2000".to_string(),
            pictures: vec![
                "https://picsum.photos/id/2/500/700".to_string(),
                "https://picsum.photos/id/3/500/700".to_string(),
            ],
            vendor_link: "https://google.com".to_string(),
            electric: false,
            type_: BikeType::Hybrid,
        },
        MyObj {
            id: "3".to_string(),
            name: "My Name 3".to_string(),
            brand: "Bar Brand".to_string(),
            price: "580".to_string(),
            pictures: vec![
                "https://picsum.photos/id/4/500/700".to_string(),
                "https://picsum.photos/id/5/500/700".to_string(),
            ],
            vendor_link: "https://google.com".to_string(),
            electric: true,
            type_: BikeType::Road,
        },
    ]))
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(bikes))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
