use std::fs::File;

use actix_web::{get, middleware::Logger, App, HttpResponse, HttpServer};
use api::routes::init_routes;

#[get("/")]
async fn index() -> Result<HttpResponse, actix_web::Error> {
    Ok(HttpResponse::Ok().body("Hello, world!"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    simplelog::CombinedLogger::init(vec![
        simplelog::TermLogger::new(
            simplelog::LevelFilter::Info,
            simplelog::Config::default(),
            simplelog::TerminalMode::Mixed,
            simplelog::ColorChoice::Auto,
        ),
        simplelog::WriteLogger::new(
            simplelog::LevelFilter::Info,
            simplelog::Config::default(),
            File::create("server.log").unwrap(),
        ),
    ])
    .unwrap();

    HttpServer::new(|| {
        App::new()
            .service(index)
            .configure(init_routes)
            .wrap(Logger::default())
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
