use super::request::*;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct TimeResponse {
    unixtime: u64,
    rfc1123: String,
}

pub async fn time() -> Result<TimeResponse, Error> {
    let response = public_request("/0/public/Time", &[]).await?;
    return load_response(&response);
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SystemStatusResponse {
    status: String,
    timestamp: String,
}

pub async fn system_status() -> Result<SystemStatusResponse, Error> {
    let response = public_request("/0/public/SystemStatus", &[]).await?;
    return load_response(&response);
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Asset {
    aclass: String,
    altname: String,
    decimals: u64,
    display_decimals: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssetsResponse(HashMap<String, Asset>);

pub async fn assets(asset: Option<&[&str]>, aclass: Option<&str>) -> Result<AssetsResponse, Error> {
    let mut params: Vec<(&str, &str)> = vec![];
    let asset_string;
    if let Some(val) = asset {
        asset_string = val.join(",");
        params.push(("asset", &asset_string));
    }
    let aclass_string;
    if let Some(val) = aclass {
        aclass_string = val;
        params.push(("aclass", aclass_string));
    }
    let response = public_request("/0/public/Assets", &params).await?;
    return load_response(&response);
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
pub struct AssetPairResponse(HashMap<String, AssetPair>);

pub async fn asset_pair(pair: &[&str], info: Option<&str>) -> Result<AssetPairResponse, Error> {
    let mut params: Vec<(&str, &str)> = vec![];
    let pair = pair.join(",");
    params.push(("pair", &pair));
    if let Some(val) = info {
        params.push(("info", val));
    }
    let response = public_request("/0/public/AssetPairs", &params).await?;
    return load_response(&response);
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssetTickerInfo {
    a: (Decimal, Decimal, Decimal),
    b: (Decimal, Decimal, Decimal),
    c: (Decimal, Decimal),
    v: (Decimal, Decimal),
    p: (Decimal, Decimal),
    t: (Decimal, Decimal),
    l: (Decimal, Decimal),
    h: (Decimal, Decimal),
    o: Decimal,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TickerResponse(HashMap<String, AssetTickerInfo>);

pub async fn ticker(pair: &str) -> Result<TickerResponse, Error> {
    let response = public_request("/0/public/Ticker", &[("pair", pair)]).await?;
    return load_response(&response);
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OHLC {
    Last(u64),
    Pair(OHLCTickData),
}

pub type OHLCTickData = Vec<(
    u64,     // time
    Decimal, // open
    Decimal, // high
    Decimal, // low
    Decimal, // close
    Decimal, // vwap
    Decimal, // volume
    u64,     // count
)>;

#[derive(Debug, Serialize, Deserialize)]
pub struct OHLCResponse(HashMap<String, OHLC>);

pub async fn ohcl(
    pair: &str,
    interval: Option<u64>,
    since: Option<u64>,
) -> Result<OHLCResponse, Error> {
    let mut params = vec![("pair", pair)];
    let interval_string;
    if let Some(val) = interval {
        interval_string = val.to_string();
        params.push(("interval", &interval_string))
    }
    let since_string;
    if let Some(val) = since {
        since_string = val.to_string();
        params.push(("since", &since_string))
    }
    let response = public_request("/0/public/OHLC", &params).await?;
    return load_response(&response);
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderBook {
    asks: Vec<(
        Decimal, // price
        Decimal, // volume
        u64,     // timestamp
    )>,
    bids: Vec<(
        Decimal, // price
        Decimal, // volume
        u64,     // timestamp
    )>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DepthResponse(HashMap<String, OrderBook>);

pub async fn depth(pair: &str, count: Option<i64>) -> Result<DepthResponse, Error> {
    let mut params = vec![("pair", pair)];
    let count_str;
    if let Some(val) = count {
        count_str = val.to_string();
        params.push(("count", &count_str));
    }
    let response = public_request("/0/public/Depth", &params).await?;
    return load_response(&response);
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Trade {
    Last(String),
    Pair(TradeTickData),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TradeTickData(Vec<(Decimal, Decimal, f64, String, String, String)>);

#[derive(Debug, Serialize, Deserialize)]
pub struct TradesResponse(HashMap<String, Trade>);

pub async fn trades(pair: &str, since: Option<i64>) -> Result<TradesResponse, Error> {
    let mut params = vec![("pair", pair)];
    let since_string;
    if let Some(val) = since {
        since_string = val.to_string();
        params.push(("since", &since_string))
    }
    let response = public_request("/0/public/Trades", &params).await?;
    return load_response(&response);
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Spread {
    Last(u64),
    Pair(SpreadData),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpreadData(Vec<(u64, Decimal, Decimal)>);

#[derive(Debug, Serialize, Deserialize)]
pub struct SpreadResponse(HashMap<String, Spread>);

pub async fn spread(pair: &str, since: Option<i64>) -> Result<SpreadResponse, Error> {
    let mut params = vec![("pair", pair)];
    let since_string;
    if let Some(val) = since {
        since_string = val.to_string();
        params.push(("since", &since_string))
    }
    let response = public_request("/0/public/Spread", &params).await?;
    return load_response(&response);
}
