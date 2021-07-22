use actix_web::{get, web, App, HttpServer, Responder};

#[get("/{id}/{name}/index.html")]
async fn index(web::Path((id, name)): web::Path<(u32, String)>) -> impl Responder {
    format!("Hello {}! id:{}", name, id)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let bind = "127.0.0.1:3000";

    println!("Starting server at: {}", &bind);
    HttpServer::new(|| App::new().service(index))
        .bind(&bind)?
        .run()
        .await
}
