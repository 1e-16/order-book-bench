use ntex::web;
use crate::id::IdGen;
use crate::scylla::get_scylla_session;

use crate::model::*;

#[web::get("/order")]
async fn get_order() -> Result<impl web::Responder, web::Error> {
    let orders = Order::rand(5);
    Ok(web::HttpResponse::Ok().json(&orders))
}

#[web::post("/order")]
async fn add_order(body: String) -> Result<impl web::Responder, web::Error> {
    let req: AddOrderReq = serde_json::from_str(&body)?;
    let order_id = IdGen::ins().gen();
    let direction = req.direction;
    let currency = req.currency;
    let amount = req.amount;
    let remark = req.remark;
    let create_at = IdGen::current_timestamp();

    let session = get_scylla_session().await;
    let ret = session.query(
        "INSERT INTO biz.orders (order_id, direction,
        currency,
        initiator,
        associate,
        amount,
        remark,
        create_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
        (
            order_id as i64,
            direction as i8,
            format!("{:?}", currency),
            0_i64,
            1_i64,
            &amount,
            &remark,
            create_at as i64),
    ).await;

    println!("{:?}", ret);

    let ord = AddOrderRsp {
        order_id,
        direction,
        currency,
        amount,
        remark,
        create_at,
    };

    Ok(web::HttpResponse::Ok().json(&ord))
}