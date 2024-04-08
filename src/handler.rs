use ntex::web;
use web::types::*;
use crate::id::IdGen;
use crate::scylla::{Insert, Query};
use crate::scylla::Op::{Eq, GtE, In, LtE};

use crate::model::*;
use crate::scylla::Ord::Desc;

#[web::post("/order/id")]
async fn get_order_by_id(req: Json<GetOrderByOIdReq>) -> Result<impl web::Responder, web::Error> {
    let q = Query::new("biz", "orders")
        .selects(["oid","side","ccy","mch","usr","amt","rmk","ctm"])
        .wheres([Eq("oid", req.oid)])
        .finish().await?;

    let rsp = GetOrderRsp {
        ords: q.collect(),
        ..Default::default()
    };

    Ok(web::HttpResponse::Ok().json(&rsp))
}

#[web::post("/order/ids")]
async fn get_order_by_ids(req: Json<GetOrderByOIdsReq>) -> Result<impl web::Responder, web::Error> {
    let q = Query::new("biz", "orders")
        .selects(["oid","side","ccy","mch","usr","amt","rmk","ctm"])
        .wheres([In("oid", &req.oids)])
        .limit(req.oids.len())
        .allow_filtering()
        .finish().await?;

    let rsp = GetOrderRsp {
        ords: q.collect(),
        ..Default::default()
    };

    Ok(web::HttpResponse::Ok().json(&rsp))
}

#[web::post("/order/usr")]
async fn get_order_by_usr(req: Json<GetOrderByUsrReq>) -> Result<impl web::Responder, web::Error> {
    let mut q = Query::new("biz", "orders");
    q.selects(["oid","side","ccy","mch","usr","amt","rmk","ctm"]).allow_filtering();

    q.wheres([Eq("usr", req.usr)]);
    if let Some(t) = req.tm_start {
        q.wheres([GtE("ctm", t)]);
    }

    if let Some(t) = req.tm_end {
        q.wheres([LtE("ctm", t)]);
    }

    if let Some(t) = req.ccy.as_ref() {
        if t.len() > 0 {
            q.wheres([In("ccy", t)]);
        }
    }

    if let Some(s) = req.side {
        q.wheres([Eq("side", s)]);
    }

    if let Some(l) = req.limit {
        q.limit(l);
    }

    if let Some(r) = req.reverse {
        if r {
            q.orders([Desc("ctm")]);
        }
    }

    let rsp = GetOrderRsp {
        ords: q.finish().await?.collect(),
        ..Default::default()
    };

    Ok(web::HttpResponse::Ok().json(&rsp))
}

#[web::post("/order/mch")]
async fn get_order_by_mch(req: Json<GetOrderByMCHReq>) -> Result<impl web::Responder, web::Error> {
    let mut q = Query::new("biz", "orders_by_mch");
    q.selects(["oid","side","ccy","mch","usr","amt","rmk","ctm"]).allow_filtering();

    q.wheres([Eq("mch", req.mch)]);
    if let Some(t) = req.tm_start {
        q.wheres([GtE("ctm", t)]);
    }

    if let Some(t) = req.tm_end {
        q.wheres([LtE("ctm", t)]);
    }

    if let Some(t) = req.ccy.as_ref() {
        if t.len() > 0 {
            q.wheres([In("ccy", t)]);
        }
    }

    if let Some(s) = req.side {
        q.wheres([Eq("side", s)]);
    }

    if let Some(l) = req.limit {
        q.limit(l);
    }

    if let Some(r) = req.reverse {
        if r {
            q.orders([Desc("ctm")]);
        }
    }

    let rsp = GetOrderRsp {
        ords: q.finish().await?.collect(),
        ..Default::default()
    };

    Ok(web::HttpResponse::Ok().json(&rsp))
}

#[web::post("/order")]
async fn add_order(req: Json<AddOrderReq>) -> Result<impl web::Responder, web::Error> {
    let oid = IdGen::ins().gen();
    let create_at = IdGen::current_timestamp();

    // TODO: 根据header识别请求方角色, 并更改target
    let usr = 1_i64;
    let mch = req.tar;

    // TODO: 根据请求方角色确认side
    let side = Side::UM;

    let ret = Insert::new("biz", "orders", &Order{
        oid,
        ctm: create_at,
        usr,
        mch,
        side,
        ..Default::default()
    }).finish().await;

    if let Err(err) = ret  {
        println!("Error inserting order: {:?}", err);
        return Ok(web::HttpResponse::InternalServerError().finish());
    }

    let ord = AddOrderRsp {
        oid,
        side,
        ccy: req.ccy,
        mch,
        usr,
        amt: req.amt.as_str().into(),
        rmk: req.rmk.as_str().into(),
        ctm: create_at,
    };

    Ok(web::HttpResponse::Ok().json(&ord))
    // if ret.is_err() {
    //     println!("Error inserting order: {:?}", ret.unwrap_err());
    //     return Ok(web::HttpResponse::InternalServerError().finish());
    // }

    // let session = get_scylla_session().await;
    // let ret = session.query(
    //     "INSERT INTO biz.orders
    //     (
    //     oid,
    //     side,
    //     ccy,
    //     mch,
    //     usr,
    //     amt,
    //     rmk,
    //     ctm
    //     ) VALUES (
    //     ?, ?, ?, ?, ?, ?, ?, ?
    //     )",
    //     (
    //         oid,
    //         side.to_string(),
    //         req.ccy.to_string(),
    //         mch,
    //         usr,
    //         &req.amt,
    //         &req.rmk,
    //         create_at),
    // ).await;
    //
    // if ret.is_err() {
    //     println!("Error inserting order: {:?}", ret.unwrap_err());
    //     return Ok(web::HttpResponse::InternalServerError().finish());
    // }

    // let ord = AddOrderRsp {
    //     oid,
    //     side,
    //     ccy: req.ccy,
    //     mch,
    //     usr,
    //     amt: req.amt.to_string(),
    //     rmk: req.rmk.to_string(),
    //     ctm: create_at,
    // };
    //
    // Ok(web::HttpResponse::Ok().json(&ord))
}