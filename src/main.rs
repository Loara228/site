use actix_web::{get, App, HttpResponse, HttpServer, Responder};


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    
    let server = HttpServer::new(move || {
        App::new()
            .service(index)
            .service(actix_files::Files::new("/", "./src/source/.").show_files_listing())
    });

    server
        .bind(("127.0.0.1", 8080))
        .unwrap()
        .run()
        .await
}

#[get("/test")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("test")
}