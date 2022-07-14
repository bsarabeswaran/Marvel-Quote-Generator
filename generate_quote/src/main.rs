extern crate actix_web;
extern crate actix_cors;

use std::io;

mod gen_quote;

#[actix_web::main]
async fn main() -> io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    actix_web::HttpServer::new(|| {
        let cors = actix_cors::Cors::permissive();
        actix_web::App::new()
            .wrap(cors)
            .service(gen_quote::gen_quote_endpoint)
    })
    .bind(("127.0.0.1", 5000))
    .unwrap()
    .run()
    .await
}