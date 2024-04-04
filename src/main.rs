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
            .service(handler::add_order)
            .service(handler::get_order_by_id)
            .service(handler::get_order_by_ids)
            .service(handler::get_order_by_usr)
            .service(handler::get_order_by_mch)
    })
        .bind(("0.0.0.0", 8081))?
        .run()
        .await
}