use super::request::*;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub async fn time() -> Result<Time, Error> {
    let response = public_request("/0/public/Time", &[]).await?;
    return load_response(&response);
}

pub async fn system_status() -> Result<SystemStatus, Error> {
    let response = public_request("/0/public/SystemStatus", &[]).await?;
    return load_response(&response);
}

pub async fn assets(asset: &[&str], aclass: &str) -> Result<HashMap<String, Asset>, Error> {
    let mut params: Vec<(&str, &str)> = vec![];
    let asset = asset.join(",");
    if asset != "" {
        params.push(("asset", &asset));
    }
    if aclass != "" {
        params.push(("aclass", aclass));
    }
    let response = public_request("/0/public/Assets", &params).await?;
    return load_response(&response);
}

pub async fn asset_pair(pair: &[&str], info: &str) -> Result<HashMap<String, AssetPair>, Error> {
    let pair = pair.join(",");
    let info = if info == "" { "info" } else { info };
    let params: Vec<(&str, &str)> = vec![("pair", &pair), ("info", info)];
    let response = public_request("/0/public/AssetPairs", &params).await?;
    return load_response(&response);
}

pub async fn ticker(pair: &str) -> Result<HashMap<String, Ticker>, Error> {
    let response = public_request("/0/public/Ticker", &[("pair", pair)]).await?;
    return load_response(&response);
}

pub async fn ohcl(
    pair: &str,
    interval: u64,
    since: Option<u64>,
) -> Result<HashMap<String, OHLC>, Error> {
    let interval = interval.to_string();
    let mut params = vec![("pair", pair), ("interval", &interval)];
    // since doesn't live long enough without it.
    let since_string;
    if let Some(val) = since {
        since_string = val.to_string();
        params.push(("since", &since_string))
    }
    let response = public_request("/0/public/OHLC", &params).await?;
    return load_response(&response);
}

pub async fn depth(pair: &str, count: Option<i64>) -> Result<HashMap<String, Depth>, Error> {
    let mut params = vec![("pair", pair)];
    let mut count_str = "100".to_string();
    if let Some(val) = count {
        count_str = val.to_string();
    }
    params.push(("count", &count_str));
    let response = public_request("/0/public/Depth", &params).await?;
    return load_response(&response);
}

pub async fn trades(pair: &str, since: Option<i64>) -> Result<HashMap<String, Trade>, Error> {
    let mut params = vec![("pair", pair)];
    let since_string;
    if let Some(val) = since {
        since_string = val.to_string();
        params.push(("since", &since_string))
    }
    let response = public_request("/0/public/Trades", &params).await?;
    return load_response(&response);
}

pub async fn spread(pair: &str, since: Option<i64>) -> Result<HashMap<String, Spread>, Error> {
    let mut params = vec![("pair", pair)];
    let since_string;
    if let Some(val) = since {
        since_string = val.to_string();
        params.push(("since", &since_string))
    }
    let response = public_request("/0/public/Spread", &params).await?;
    return load_response(&response);
}

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
