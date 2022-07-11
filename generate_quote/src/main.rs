extern crate actix_web;

use std::io;

mod gen_quote;

#[actix_web::main]
async fn main() -> io::Result<()> {
    actix_web::HttpServer::new(|| {
        actix_web::App::new()
            .service(gen_quote::gen_quote_endpoint)
    })
    .bind(("127.0.0.1", 5000))?
    .run()
    .await
}