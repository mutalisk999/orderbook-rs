// #[macro_use]
use erased_serde::serialize_trait_object;
use serde::{Deserialize, Serialize};
use serde_json;

use crate::matching::order_book::BookOrder;
use crate::models::types::{DoneReason, Side, TimeInForceType};
use chrono::prelude::*;
use rust_decimal::Decimal;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum LogType {
    LogTypeMatch,
    LogTypeOpen,
    LogTypeDone,
}

pub trait Log: erased_serde::Serialize {
    fn get_seq(&self) -> u64;
}

serialize_trait_object!(Log);

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Base {
    pub r#type: LogType,
    pub sequence: u64,
    pub product_id: String,
    #[serde(with = "chrono::serde::ts_nanoseconds")]
    pub time: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OpenLog {
    pub base: Base,
    pub order_id: u64,
    pub user_id: u64,
    pub remaining_size: Decimal,
    pub price: Decimal,
    pub side: Side,
    pub time_in_force: TimeInForceType,
}

impl Log for OpenLog {
    fn get_seq(&self) -> u64 {
        self.base.sequence
    }
}

pub fn new_open_log(log_seq: u64, product_id: &str, taker_order: &BookOrder) -> OpenLog {
    OpenLog {
        base: Base {
            r#type: LogType::LogTypeOpen,
            sequence: log_seq,
            product_id: product_id.to_string(),
            time: Utc::now(),
        },
        order_id: taker_order.order_id,
        user_id: taker_order.user_id,
        remaining_size: taker_order.size,
        price: taker_order.price,
        side: taker_order.side.clone(),
        time_in_force: taker_order.time_in_force.clone(),
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DoneLog {
    pub base: Base,
    pub order_id: u64,
    pub user_id: u64,
    pub price: Decimal,
    pub remaining_size: Decimal,
    pub reason: DoneReason,
    pub side: Side,
    pub time_in_force: TimeInForceType,
}

impl Log for DoneLog {
    fn get_seq(&self) -> u64 {
        self.base.sequence
    }
}

pub fn new_done_log(
    log_seq: u64,
    product_id: &str,
    order: &BookOrder,
    remaining_size: &Decimal,
    reason: &DoneReason,
) -> DoneLog {
    DoneLog {
        base: Base {
            r#type: LogType::LogTypeDone,
            sequence: log_seq,
            product_id: product_id.to_string(),
            time: Utc::now(),
        },
        order_id: order.order_id,
        user_id: order.user_id,
        price: order.price,
        remaining_size: remaining_size.clone(),
        reason: reason.clone(),
        side: order.side.clone(),
        time_in_force: order.time_in_force.clone(),
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MatchLog {
    pub base: Base,
    pub trade_seq: u64,
    pub taker_order_id: u64,
    pub maker_order_id: u64,
    pub taker_user_id: u64,
    pub maker_user_id: u64,
    pub side: Side,
    pub price: Decimal,
    pub size: Decimal,
    pub taker_time_in_force: TimeInForceType,
    pub maker_time_in_force: TimeInForceType,
}

impl Log for MatchLog {
    fn get_seq(&self) -> u64 {
        self.base.sequence
    }
}

pub fn new_match_log(
    log_seq: u64,
    product_id: &str,
    trade_seq: u64,
    taker_order: &BookOrder,
    maker_order: &BookOrder,
    price: &Decimal,
    size: &Decimal,
) -> MatchLog {
    MatchLog {
        base: Base {
            r#type: LogType::LogTypeMatch,
            sequence: log_seq,
            product_id: product_id.to_string(),
            time: Utc::now(),
        },
        trade_seq,
        taker_order_id: taker_order.order_id,
        maker_order_id: maker_order.order_id,
        taker_user_id: taker_order.user_id,
        maker_user_id: maker_order.user_id,
        side: maker_order.side.clone(),
        price: price.clone(),
        size: size.clone(),
        taker_time_in_force: taker_order.time_in_force.clone(),
        maker_time_in_force: maker_order.time_in_force.clone(),
    }
}
