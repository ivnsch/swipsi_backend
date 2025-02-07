use actix_web::{get, web, App, HttpServer, Responder, Result};
use serde::Serialize;

#[derive(Serialize)]
struct MyObj {
    id: String,
    name: String,
    brand: String,
    price: String,
    pictures: Vec<String>,
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
                "https://picsum.photos/id/0/200/300".to_string(),
                "https://picsum.photos/id/1/200/300".to_string(),
            ],
        },
        MyObj {
            id: "2".to_string(),
            name: "My Name 2".to_string(),
            brand: "Bar Brand".to_string(),
            price: "2000".to_string(),
            pictures: vec![
                "https://picsum.photos/id/2/200/300".to_string(),
                "https://picsum.photos/id/3/200/300".to_string(),
            ],
        },
        MyObj {
            id: "3".to_string(),
            name: "My Name 3".to_string(),
            brand: "Bar Brand".to_string(),
            price: "580".to_string(),
            pictures: vec![
                "https://picsum.photos/id/4/200/300".to_string(),
                "https://picsum.photos/id/5/200/300".to_string(),
            ],
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
