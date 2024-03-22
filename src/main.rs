mod id;
mod mask;
mod scylla;
mod handler;
mod model;

use std::io::Read;

use ntex::web;

#[ntex::main]
async fn main() -> std::io::Result<()> {
    web::HttpServer::new(|| {
        web::App::new()
            .service(handler::get_order)
            .service(handler::add_order)
    })
        .bind(("0.0.0.0", 8081))?
        .run()
        .await
}