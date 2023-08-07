use actix_web::{HttpServer, App, web, Responder, get};

#[get("/")]
async fn index() -> impl Responder {
    "Hello World!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .service(
                web::scope("/api")
                    .service(index)
            )
            .service(
                actix_files::Files::new("/", String::from("/client/dist/"))
            )
    })
    .bind(("localhost", 8080))?
    .run()
    .await
}