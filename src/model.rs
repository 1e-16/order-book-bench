use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::id::IdGen;

#[derive(Debug, EnumIter, Serialize, Deserialize)]
#[derive(Copy, Clone)]
#[repr(i8)]
pub enum Transaction {
    // #[serde(rename = "出金")]
    Withdrawal,
    // #[serde(rename = "入金")]
    Deposit,
    // Unknown,
}

#[derive(Debug, EnumIter, Serialize, Deserialize)]
#[derive(Copy, Clone)]
pub enum Currency {
    // #[serde(rename = "CNY")]
    CNY,
    // #[serde(rename = "USD")]
    USD,
    // #[serde(rename = "EUR")]
    EUR,
    // #[serde(rename = "BTC")]
    BTC,
    // Unknown,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Order {
    pub oid: u64,
    pub txn: Transaction,
    pub ccy: Currency,
    pub amt: String,
    pub rmk: String,
    pub ctm: u64,
}

impl Order {
    pub fn rand(cnt: u16) -> Vec<Order> {
        let mut orders = Vec::new();

        for _ in 0..cnt {
            let oid = IdGen::ins().gen();
            let txn = fastrand::choice(Transaction::iter()).unwrap();
            let ccy = fastrand::choice(Currency::iter()).unwrap();
            let amt = fastrand::u128(1..=1_000_000_000).to_string();
            let rmt = oid.to_string();
            let ctm = IdGen::current_timestamp();

            let order = Order {
                oid,
                txn,
                ccy,
                amt,
                rmk,
                ctm,
            };

            orders.push(order);
        }

        orders
    }
}

#[derive(Deserialize)]
#[derive(Debug)]
pub struct AddOrderReq {
    pub txn: Transaction,
    pub ccy: Currency,
    pub amt: String,
    pub rmk: String,
}

pub type AddOrderRsp = Order;

#[derive(Deserialize)]
#[derive(Debug)]
pub struct GetOrderByOIdReq {
    // 指定订单ID
    pub oid: u64,
}

#[derive(Deserialize)]
#[derive(Debug)]
pub struct GetOrderByOIdsReq {
    // 指定订单ID
    pub oid: Vec<u64>,
    // 是否反方向
    pub reverse: bool,
}

#[derive(Deserialize)]
#[derive(Debug)]
pub struct GetOrderByEUReq {
    // 终端用户
    pub usr: u64,

    // 指定货币&方向
    pub ccy: Currency,
    pub txn: Transaction,

    // 时间范围指定
    pub tm_start: u64,
    pub tm_end: u64,

    // 批量查询选项
    // 查询数量
    pub limit: usize,
    // 从那个ID开始查起
    pub from_oid: u64,
    // 是否反方向
    pub reverse: bool,
}

#[derive(Deserialize)]
#[derive(Debug)]
pub struct GetOrderByMCHReq {
    // 商户
    pub mch: u64,

    // 指定货币&方向
    pub ccy: Currency,
    pub txn: Transaction,

    // 时间范围指定
    pub tm_start: u64,
    pub tm_end: u64,

    // 批量查询选项
    // 查询数量
    pub limit: usize,
    // 从那个ID开始查起
    pub from_oid: u64,
    // 是否反方向
    pub reverse: bool,
}

#[derive(Deserialize)]
#[derive(Debug)]
pub struct GetOrderReq {
    // 终端用户
    pub usr: u64,

    // 指定货币&方向
    pub ccy: Currency,
    pub txn: Transaction,

    // 指定订单ID
    pub oid: u64,

    // 商户
    pub mch: u64,

    // 时间范围指定
    pub start: u64,
    pub end: u64,

    // 批量查询选项
    // 查询数量
    pub limit: usize,
    // 从那个ID开始查起
    pub from_oid: u64,
    // 是否反方向
    pub reverse: bool,
}

#[derive(Serialize)]
pub struct GetOrderRsp(Vec<Order>);