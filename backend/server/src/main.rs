use std::fs;

use ntex::web;

#[ntex::main]
async fn main() -> std::io::Result<()> {
    web::HttpServer::new(|| web::App::new().service(get_spaces))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

#[web::post("/get-spaces")]
async fn get_spaces() -> impl web::Responder {
    serde_json::from_str::<Vec<String>>(&fs::read_to_string("spaces.json").unwrap())
        .map(|spaces| web::Json(spaces))
        .unwrap_or_else(|_| web::Json(vec![]))
}
