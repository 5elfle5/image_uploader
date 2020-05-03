use actix_web::{middleware, App, HttpServer};
use actix_web::web::{resource, get, post};
mod handlers;
mod file_utils;
mod errors;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::fs::create_dir_all("./tmp").unwrap();
    std::fs::create_dir_all("./tmp/preview").unwrap();

    HttpServer::new(|| {
        App::new().wrap(middleware::Logger::default()).service(
            resource("/")
                .route(get().to(handlers::index))
                .route(post().to(handlers::save_file)),
        )
    })
    .bind("0.0.0.0:3000")?
    .run()
    .await
}
