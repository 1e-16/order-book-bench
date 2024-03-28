use ntex::web;
use web::types::*;
use crate::id::IdGen;
use crate::scylla::get_scylla_session;

use crate::model::*;

#[web::get("/order")]
async fn get_order(Query(req): Query<AddOrderReq>) -> Result<impl web::Responder, web::Error> {
    println!("{:?}", req);
    Ok(web::HttpResponse::Ok())
}

#[web::post("/order")]
async fn add_order(body: String) -> Result<impl web::Responder, web::Error> {
    let req: AddOrderReq = serde_json::from_str(&body)?;
    let order_id = IdGen::ins().gen();
    let direction = req.txn;
    let currency = req.ccy;
    let amount = req.amt;
    let remark = req.rmk;
    let create_at = IdGen::current_timestamp();

    let session = get_scylla_session().await;
    let ret = session.query(
        "INSERT INTO biz.orders (order_id, direction,
        currency,
        initiator,
        usr,
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
        oid: order_id,
        txn: direction,
        ccy: currency,
        amt: amount,
        remark,
        create_at,
    };

    Ok(web::HttpResponse::Ok().json(&ord))
}