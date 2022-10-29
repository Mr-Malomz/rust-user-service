use actix_web::{App, HttpServer};
use api::handlers::{create_user, get_user};

mod api;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || App::new().service(get_user).service(create_user))
        .bind(("localhost", 8080))?
        .run()
        .await
}
