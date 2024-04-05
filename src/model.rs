use std::str::FromStr;
use serde::{Deserialize, Serialize, Serializer};
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter, EnumString};
use scylla::FromRow;
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};
use scylla::_macro_internal::{FromRowError, Row};
use scylla::transport::session::TypedRowIter;

use crate::id::IdGen;
use crate::scylla::{Op, Quoted};

#[derive(Debug, Display)]
#[derive(PartialEq)]
#[derive(Copy, Clone)]
#[derive(EnumIter, EnumString)]
#[derive(Serialize, Deserialize)]
#[derive(FromPrimitive, ToPrimitive)]
pub enum Side {
    All = 0,
    // 商户->用户
    MU = 1,
    // 用户->商户
    UM = -1,
}

impl<'a> Quoted for &'a Side {
    fn is_quoted() -> bool {
        true
    }
}

impl Quoted for Side {
    fn is_quoted() -> bool {
        true
    }
}

impl Default for Side {
    fn default() -> Self {
        Side::All
    }
}

impl Into<String> for Side {
    fn into(self) -> String {
        self.to_string()
    }
}

#[derive(Debug, Display)]
#[derive(Copy, Clone)]
#[derive(EnumIter, EnumString)]
#[derive(Serialize, Deserialize)]
#[derive(FromPrimitive, ToPrimitive)]
pub enum Ccy {
    CNY,
    USD,
    EUR,
    BTC,
}

impl Into<String> for Ccy {
    fn into(self) -> String {
        self.to_string()
    }
}

impl<'a> Quoted for &'a Ccy {
    fn is_quoted() -> bool {
        true
    }
}

impl Quoted for Ccy {
    fn is_quoted() -> bool {
        true
    }
}

impl Default for Ccy {
    fn default() -> Self {
        Ccy::CNY
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[derive(Default)]
pub struct Order {
    pub oid: i64,
    pub side: Side,
    pub ccy: Ccy,
    pub mch: i64,
    pub usr: i64,
    pub amt: String,
    pub ctm: i64,
    pub rmk: String,
}

impl FromRow for Order {
    fn from_row(row: Row) -> Result<Self, FromRowError> {
        let oid = row.columns[0].as_ref().unwrap().as_bigint().unwrap();
        let side = Side::from_str(row.columns[1].as_ref().unwrap().as_text().unwrap()).unwrap();
        let ccy = Ccy::from_str(row.columns[2].as_ref().unwrap().as_text().unwrap()).unwrap();
        let mch = row.columns[3].as_ref().unwrap().as_bigint().unwrap();
        let usr = row.columns[4].as_ref().unwrap().as_bigint().unwrap();
        let amt = row.columns[5].as_ref().unwrap().as_text().unwrap().to_string();
        let rmk = row.columns[6].as_ref().unwrap().as_text().unwrap().to_string();
        let ctm = row.columns[7].as_ref().unwrap().as_bigint().unwrap();
        Ok(Order {
            oid,
            side,
            ccy,
            mch,
            usr,
            amt,
            rmk,
            ctm,
        })
    }
}

impl Order {
    pub fn rand(cnt: u16) -> Vec<Order> {
        let mut orders = Vec::new();

        for _ in 0..cnt {
            let oid = IdGen::ins().gen();
            let side = fastrand::choice(Side::iter()).unwrap();
            let ccy = fastrand::choice(Ccy::iter()).unwrap();
            let amt = fastrand::u128(1..=1_000_000_000).to_string();
            let rmk = oid.to_string();
            let ctm = IdGen::current_timestamp();

            let order = Order {
                oid,
                side,
                ccy,
                mch: 0,
                usr: 0,
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
    pub tar: i64,
    pub ccy: Ccy,
    pub amt: String,
    pub rmk: String,
}

pub type AddOrderRsp = Order;

#[derive(Deserialize)]
#[derive(Debug)]
pub struct GetOrderByOIdReq {
    // 指定订单ID
    pub oid: i64,
}

#[derive(Deserialize)]
#[derive(Debug)]
pub struct GetOrderByOIdsReq {
    // 指定订单ID
    pub oids: Vec<i64>,
    // 是否反方向
    pub reverse: Option<bool>,
}

#[derive(Deserialize)]
#[derive(Debug)]
pub struct GetOrderByUsrReq {
    // 终端用户
    pub usr: i64,

    // 指定货币
    pub ccy: Option<Vec<Ccy>>,

    // 指定方向
    pub side: Option<Side>,

    // 时间范围指定
    pub tm_start: Option<i64>,
    pub tm_end: Option<i64>,

    // 批量查询选项
    // 查询数量
    pub limit: Option<usize>,
    // 从指定位置开始
    pub cursor: Option<String>,
    // 是否反方向
    pub reverse: Option<bool>,
}

#[derive(Deserialize)]
#[derive(Debug)]
pub struct GetOrderByMCHReq {
    // 商户
    pub mch: i64,

    // 指定货币
    pub ccy: Option<Vec<Ccy>>,

    // 指定方向
    pub side: Option<Side>,

    // 时间范围指定
    pub tm_start: Option<i64>,
    pub tm_end: Option<i64>,

    // 批量查询选项
    // 查询数量
    pub limit: Option<usize>,
    // 从指定位置开始
    pub cursor: Option<String>,
    // 是否反方向
    pub reverse: Option<bool>,
}

#[derive(Serialize)]
#[derive(Default)]
pub struct GetOrderRsp {
    pub ords: Vec<Order>,
    // 游标, 用于翻页时位置记录
    pub cursor: String,
}