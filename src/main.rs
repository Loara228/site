use actix_web::{App, HttpServer};


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    
    let server = HttpServer::new(move || {
        App::new()
            .service(actix_files::Files::new("/", "src/static").show_files_listing())
        
    });

    server
        .bind(("127.0.0.1", 8080))
        .unwrap()
        .run()
        .await
}