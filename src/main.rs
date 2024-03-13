mod id;

use std::io;
use std::time::{Duration, SystemTime};
use bytes::BufMut;

use may_minihttp::{HttpService, HttpServiceFactory, Request, Response};
use serde::{Serialize, Deserialize};
use crate::id::IdGen;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, EnumIter, Serialize, Deserialize)]
#[derive(Copy, Clone)]
enum Direction {
    #[serde(rename = "出金")]
    Withdrawal,
    #[serde(rename = "入金")]
    Deposit,
}

// impl Distribution<Direction> for Standard {
//     fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Direction {
//         match rng.gen_range(0..Direction::Unknown as usize) { // rand 0.8.5
//             0 => Direction::Withdrawal,
//             1 => Direction::Deposit,
//             _ => Direction::Unknown,
//         }
//     }
// }

#[derive(Debug, EnumIter, Serialize, Deserialize)]
#[derive(Copy, Clone)]
enum Currency {
    #[serde(rename = "CNY")]
    CNY,
    #[serde(rename = "USD")]
    USD,
    #[serde(rename = "EUR")]
    EUR,
    #[serde(rename = "BTC")]
    BTC,
}

// impl Distribution<Currency> for Standard {
//     fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Currency {
//         match rng.gen_range(0..Currency::Unknown as usize) { // rand 0.8.5
//             0 => Currency::CNY,
//             1 => Currency::USD,
//             2 => Currency::EUR,
//             3 => Currency::BTC,
//             _ => Currency::Unknown,
//         }
//     }
// }

#[derive(Debug, Serialize, Deserialize)]
struct Order {
    order_id: u64,
    direction: Direction,
    currency: Currency,
    amount: u128,
    remark: String,
    create_at: u128,
}

// fn generate_random_order() -> Order {
//     let order_id = IdGen::ins().gen();
//     let direction = generate_random_direction();
//     let currency = generate_random_currency();
//     let amount = rand::thread_rng().gen_range(100..1000); // 生成100到1000之间的随机金额
//     let remark = "Random Order".to_string();
//     let create_at = current_timestamp();
//
//     Order {
//         order_id,
//         direction,
//         currency,
//         amount,
//         remark,
//         create_at,
//     }
// }

// fn generate_random_currency() -> Currency {
//     let currencies: &[Currency] = &[
//         Currency::CNY,
//         Currency::USD,
//         Currency::EUR,
//         Currency::BTC,
//     ];
//     let index = rand::thread_rng().gen_range(0..currencies.len());
//
//     *currencies[index]
// }

fn current_timestamp() -> u128 {
    let start = SystemTime::now();
    let since_epoch = start.duration_since(SystemTime::UNIX_EPOCH).unwrap_or(Duration::ZERO);

    since_epoch.as_nanos()
}

impl Order {
    fn rand(cnt: u16) -> Vec<Order> {
        let mut orders = Vec::new();

        for _ in 0..cnt {
            let order_id = IdGen::ins().gen();
            let direction = fastrand::choice(Direction::iter()).unwrap();
            let currency = fastrand::choice(Currency::iter()).unwrap();
            let amount = fastrand::u128(1..=1_000_000_000);
            let remark = order_id.to_string();
            let create_at = current_timestamp();

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

#[derive(Debug, Deserialize)]
struct OrderQuery {
    order_id: u64,
    direction: Direction,
    currency: Currency,
    amount: u128,
    remark: String,
    create_at: u64,
}

#[derive(Clone)]
struct OrderBook {}

impl HttpService for OrderBook {
    fn call(&mut self, req: Request, rsp: &mut Response) -> io::Result<()> {
        match req.path() {
            "/order" => self.handle_order_request(req.method(), rsp),
            _ => {
                rsp.status_code(404, "Not Found");
            }
        }

        Ok(())
    }
}

impl OrderBook {
    fn handle_order_request(&mut self, method: &str, rsp: &mut Response) {
        match method {
            "GET" => self.handle_get_order(rsp),
            "POST" => self.handle_post_order(rsp),
            _ => {
                rsp.status_code(404, "Not Found");
            }
        }
    }

    fn handle_get_order(&mut self, rsp: &mut Response) {
        rsp.header("Content-Type: application/json");

        let orders = Order::rand(5);
        serde_json::to_writer_pretty(rsp.body_mut().writer(), &orders).unwrap();
    }

    fn handle_post_order(&mut self, _rsp: &mut Response) {
        // 处理 写 请求
    }
}

struct HttpServer {}

impl HttpServiceFactory for HttpServer {
    type Service = OrderBook;

    fn new_service(&self, _: usize) -> Self::Service {
        OrderBook {}
    }
}

fn main() {
    may::config().set_pool_capacity(500).set_stack_size(0x1000);
    let http_server = HttpServer {};
    let server = http_server.start("0.0.0.0:8081").unwrap();
    server.join().unwrap();
}