use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use clap::Parser;

#[derive(Parser, Debug)]
pub struct Cli
{
    #[arg(short, long)]
    pub address: Option<String>,

    #[arg(short, long)]
    pub port: Option<u16>
}

pub struct AppData {
    
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let cli = Cli::parse();

    let addr: &str = &cli.address.unwrap_or("127.0.0.1".to_string());
    let port: u16 = cli.port.unwrap_or(8080u16);
    
    let server = HttpServer::new(move || {
        App::new()
            .service(index)
            .service(actix_files::Files::new("/", "./src/source/.").show_files_listing())
    });

    server
        .bind((addr, port))
        .unwrap()
        .run()
        .await
}

#[get("/test")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("test")
}