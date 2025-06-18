mod backend;

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use clap::Parser;
use sqlx::{PgPool, Pool, Postgres};
use tera::Tera;

#[derive(Parser, Debug)]
pub struct Cli
{
    #[arg(short, long)]
    pub address: Option<String>,

    #[arg(short, long)]
    pub port: Option<u16>
}

#[derive(Clone)]
pub struct SiteData {
    pub pool: Pool<Postgres>,
    pub tera: Tera
}

impl SiteData {
    pub async fn new() -> Self {
        let pool = PgPool::connect("postgres://admin:password@localhost:5432/site").await.unwrap();
        backend::db::create_tables(&pool).await;
        
        Self {
            pool,
            tera: Tera::new("src/frontend/**").unwrap()
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let cli = Cli::parse();

    tracing_subscriber::fmt().with_max_level(tracing::Level::INFO).init();

    let addr: &str = &cli.address.unwrap_or("127.0.0.1".to_string());
    let port: u16 = cli.port.unwrap_or(8080u16);

    let data = SiteData::new().await;
    
    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(data.clone()))
            
            .service(
                web::scope("/users")
                    .service(backend::services::users::user)
            )

            .service(actix_files::Files::new("/", "./src/frontend/.").show_files_listing())
    });

    server
        .bind((addr, port))
        .unwrap()
        .run()
        .await
}