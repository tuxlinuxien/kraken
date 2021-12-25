use rust_decimal::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Time {
    unixtime: u64,
    rfc1123: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SystemStatus {
    status: String,
    timestamp: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Asset {
    aclass: String,
    altname: String,
    decimals: u64,
    display_decimals: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssetPair {
    altname: String,
    wsname: String,
    aclass_base: String,
    base: String,
    aclass_quote: String,
    quote: String,
    lot: String,
    pair_decimals: u64,
    lot_decimals: u64,
    lot_multiplier: u64,
    leverage_buy: Vec<u64>,
    leverage_sell: Vec<u64>,
    fees: Vec<Vec<Decimal>>,
    fees_maker: Vec<Vec<Decimal>>,
    fee_volume_currency: String,
    margin_call: u64,
    margin_stop: u64,
    ordermin: Decimal,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Ticker {
    a: Vec<Decimal>,
    b: Vec<Decimal>,
    c: Vec<Decimal>,
    v: Vec<Decimal>,
    p: Vec<Decimal>,
    t: Vec<Decimal>,
    l: Vec<Decimal>,
    h: Vec<Decimal>,
    o: Decimal,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OHLC {
    Last(u64),
    Pair(
        Vec<(
            u64,
            Decimal,
            Decimal,
            Decimal,
            Decimal,
            Decimal,
            Decimal,
            u64,
        )>,
    ),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Depth {
    asks: Vec<(Decimal, Decimal, u64)>,
    bids: Vec<(Decimal, Decimal, u64)>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Trade {
    Last(String),
    Pair(Vec<(Decimal, Decimal, f64, String, String, String)>),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Spread {
    Last(u64),
    Pair(Vec<(u64, Decimal, Decimal)>),
}
