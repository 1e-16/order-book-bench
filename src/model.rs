use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::id::IdGen;

#[derive(Debug, EnumIter, Serialize, Deserialize)]
#[derive(Copy, Clone)]
#[repr(i8)]
pub enum Direction {
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
    pub order_id: u64,
    pub direction: Direction,
    pub currency: Currency,
    pub amount: String,
    pub remark: String,
    pub create_at: u64,
}

impl Order {
    pub fn rand(cnt: u16) -> Vec<Order> {
        let mut orders = Vec::new();

        for _ in 0..cnt {
            let order_id = IdGen::ins().gen();
            let direction = fastrand::choice(Direction::iter()).unwrap();
            let currency = fastrand::choice(Currency::iter()).unwrap();
            let amount = fastrand::u128(1..=1_000_000_000).to_string();
            let remark = order_id.to_string();
            let create_at = IdGen::current_timestamp();

            let order = Order {
                order_id,
                direction,
                currency,
                amount,
                remark,
                create_at,
            };

            orders.push(order);
        }

        orders
    }
}

#[derive(Deserialize)]
#[derive(Debug)]
pub struct AddOrderReq {
    pub direction: Direction,
    pub currency: Currency,
    pub amount: String,
    pub remark: String,
}

pub type AddOrderRsp = Order;

#[derive(Deserialize)]
#[derive(Debug)]
pub struct GetOrderReq {
    // 受益人
    pub associate: u64,

    // 指定货币&方向
    pub currency: Currency,
    pub direction: Direction,

    // 指定订单ID
    pub order_id: u64,

    // 订单发起者
    pub initiator: u64,

    // 时间范围指定
    pub time_begin: u64,
    pub time_end: u64,

    // 批量查询选项
    // 查询数量
    pub limit: usize,
    // 从那个ID开始查起
    pub tail_order_id: u64,
    // 是否反方向
    pub reverse: bool,
}

#[derive(Serialize)]
pub struct GetOrderRsp(Vec<Order>);