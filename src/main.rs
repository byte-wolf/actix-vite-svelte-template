use actix_web::{HttpServer, App, web, Responder, get};

#[get("/")]
async fn index() -> impl Responder {
    "Hello World!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        //print!("{}", std::env::current_dir().expect("Fisch").to_str().expect("Fisch"));
        App::new()
            .service(
                web::scope("/api")
                    .service(index)
            )
            .service(
                actix_files::Files::new("/", "./client/build")
                    //.show_files_listing()
                    .index_file("index.html")
                    .default_handler(
                        actix_files::NamedFile::open(
                            ["./client/build".to_string() , "index.html".to_string()].join("/"),
                        )
                        .expect("index file should exist"),
                    ),
            )
    })
    .bind(("localhost", 8080))?
    .run()
    .await
}