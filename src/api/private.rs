use super::request::*;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DefaultOnError};
use std::collections::HashMap;

pub type BalanceResponse = HashMap<String, Decimal>;

pub async fn balance(cred: &Credential) -> Result<BalanceResponse, Error> {
    let response = private_request(&cred, "/0/private/Balance", &[]).await?;
    return load_response(&response);
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BalanceEx {
    balance: Decimal,
    hold_trade: Decimal,
}

pub type BalanceExResponse = HashMap<String, BalanceEx>;

pub async fn balance_ex(cred: &Credential) -> Result<BalanceExResponse, Error> {
    let response = private_request(&cred, "/0/private/BalanceEx", &[]).await?;
    return load_response(&response);
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TradeBalanceResponse {
    eb: Decimal,
    tb: Decimal,
    m: Decimal,
    n: Decimal,
    c: Decimal,
    v: Decimal,
    e: Decimal,
    mf: Decimal,
    ml: Option<Decimal>,
}

pub async fn trade_balance(
    cred: &Credential,
    asset: Option<&str>,
) -> Result<TradeBalanceResponse, Error> {
    let mut params: Vec<(&str, &str)> = vec![];
    if let Some(val) = asset {
        params.push(("asset", &val));
    }
    let response = private_request(&cred, "/0/private/TradeBalance", &params).await?;
    return load_response(&response);
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderDescr {
    pair: String,
    #[serde(rename = "type")]
    type_: String, // needs to be renamed
    ordertype: String,
    price: Decimal,
    price2: Decimal,
    leverage: String,
    order: String,
    close: String,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct Order {
    refid: Option<String>,
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[serde(default)]
    userref: Option<String>,
    status: String,
    opentm: f64,
    starttm: i64,
    expiretm: i64,
    descr: OrderDescr,
    vol: Decimal,
    vol_exec: Decimal,
    cost: Decimal,
    fee: Decimal,
    price: Decimal,
    stopprice: Decimal,
    limitprice: Decimal,
    misc: String,
    oflags: String,
    trades: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenOrdersResponse {
    open: HashMap<String, Order>,
}

pub async fn open_orders(
    cred: &Credential,
    trades: Option<bool>,
    userref: Option<u32>,
) -> Result<OpenOrdersResponse, Error> {
    let mut params: Vec<(&str, &str)> = vec![];
    let trades_string;
    if let Some(val) = trades {
        trades_string = val.to_string();
        params.push(("trades", &trades_string));
    }
    let userref_string;
    if let Some(val) = userref {
        userref_string = val.to_string();
        params.push(("userref", &userref_string));
    }
    let response = private_request(&cred, "/0/private/OpenOrders", &params).await?;
    return load_response(&response);
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClosedOrdersResponse {
    closed: HashMap<String, Order>,
}

pub async fn closed_orders(
    cred: &Credential,
    trades: Option<bool>,
    userref: Option<u32>,
    start: Option<i64>,
    end: Option<i64>,
    ofs: Option<i64>,
    closetime: Option<String>,
) -> Result<ClosedOrdersResponse, Error> {
    let mut params: Vec<(&str, &str)> = vec![];
    let trades_string;
    if let Some(val) = trades {
        trades_string = val.to_string();
        params.push(("trades", &trades_string));
    }
    let userref_string;
    if let Some(val) = userref {
        userref_string = val.to_string();
        params.push(("userref", &userref_string));
    }
    let start_string;
    if let Some(val) = start {
        start_string = val.to_string();
        params.push(("start", &start_string));
    }
    let end_string;
    if let Some(val) = end {
        end_string = val.to_string();
        params.push(("end", &end_string));
    }
    let ofs_string;
    if let Some(val) = ofs {
        ofs_string = val.to_string();
        params.push(("ofs", &ofs_string));
    }
    let closetime_string;
    if let Some(val) = closetime {
        closetime_string = val;
        params.push(("closetime", &closetime_string));
    }
    let response = private_request(&cred, "/0/private/ClosedOrders", &params).await?;
    return load_response(&response);
}

pub type QueryOrdersResponse = HashMap<String, Order>;

pub async fn query_orders(
    cred: &Credential,
    trades: Option<bool>,
    userref: Option<u32>,
    txid: &[&str],
) -> Result<QueryOrdersResponse, Error> {
    let mut params: Vec<(&str, &str)> = vec![];
    let trades_string;
    if let Some(val) = trades {
        trades_string = val.to_string();
        params.push(("trades", &trades_string));
    }
    let userref_string;
    if let Some(val) = userref {
        userref_string = val.to_string();
        params.push(("userref", &userref_string));
    }
    let txid = txid.join(",");
    params.push(("txid", &txid));
    let response = private_request(&cred, "/0/private/QueryOrders", &params).await?;
    return load_response(&response);
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Trade {
    ordertxid: String,
    postxid: Option<String>,
    pair: String,
    time: f64,
    #[serde(rename = "type")]
    type_: String, // needs to be renamed
    ordertype: String,
    price: Decimal,
    cost: Decimal,
    fee: Decimal,
    vol: Decimal,
    margin: Decimal,
    misc: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TradesHistoryResponse {
    trades: HashMap<String, Trade>,
    count: u64,
}

pub async fn trades_history(
    cred: &Credential,
    type_: Option<&str>,
    trades: Option<bool>,
    start: Option<i64>,
    end: Option<i64>,
    ofs: Option<i64>,
) -> Result<TradesHistoryResponse, Error> {
    let mut params: Vec<(&str, &str)> = vec![];
    if let Some(val) = type_ {
        params.push(("type", val));
    }
    let trades_string;
    if let Some(val) = trades {
        trades_string = val.to_string();
        params.push(("trades", &trades_string));
    }
    let start_string;
    if let Some(val) = start {
        start_string = val.to_string();
        params.push(("start", &start_string));
    }
    let end_string;
    if let Some(val) = end {
        end_string = val.to_string();
        params.push(("end", &end_string));
    }
    let ofs_string;
    if let Some(val) = ofs {
        ofs_string = val.to_string();
        params.push(("ofs", &ofs_string));
    }
    let response = private_request(&cred, "/0/private/TradesHistory", &params).await?;
    return load_response(&response);
}

pub type QueryTradesResponse = HashMap<String, Trade>;

pub async fn query_trades(
    cred: &Credential,
    txids: &[&str],
    trades: Option<bool>,
) -> Result<QueryTradesResponse, Error> {
    let mut params: Vec<(&str, &str)> = vec![];
    let trades_string;
    if let Some(val) = trades {
        trades_string = val.to_string();
        params.push(("trades", &trades_string));
    }
    let txids = txids.join(",");
    if txids != "" {
        params.push(("txid", &txids))
    }
    let response = private_request(&cred, "/0/private/QueryTrades", &params).await?;
    return load_response(&response);
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenPosition {
    ordertxid: String,
    posstatus: String,
    pair: String,
    time: f64,
    #[serde(rename = "type")]
    type_: String, // needs to be renamed
    ordertype: String,
    cost: Decimal,
    fee: Decimal,
    vol: Decimal,
    vol_closed: Decimal,
    margin: Decimal,
    value: Decimal,
    net: Decimal,
    terms: String,
    rollovertm: String,
    misc: String,
    oflags: String,
}

pub type OpenPositionsResponse = HashMap<String, OpenPosition>;

pub async fn open_positions(
    cred: &Credential,
    txids: &[&str],
    docalcs: Option<bool>,
    consolidation: &str,
) -> Result<OpenPositionsResponse, Error> {
    let mut params: Vec<(&str, &str)> = vec![("consolidation", &consolidation)];
    let txids = txids.join(",");
    params.push(("txid", &txids));
    let docalcs_string;
    if let Some(val) = docalcs {
        docalcs_string = val.to_string();
        params.push(("docalcs", &docalcs_string));
    }
    let response = private_request(&cred, "/0/private/OpenPositions", &params).await?;
    return load_response(&response);
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Ledger {
    refid: String,
    time: f64,
    #[serde(rename = "type")]
    type_: String,
    subtype: String,
    aclass: String,
    asset: String,
    amount: Decimal,
    fee: Decimal,
    balance: Decimal,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LedgersResponse {
    ledger: HashMap<String, Ledger>,
}

pub async fn ledgers(
    cred: &Credential,
    asset: Option<&str>,
    aclass: Option<&str>,
    type_: Option<&str>,
    start: Option<i64>,
    end: Option<i64>,
    ofs: Option<i64>,
) -> Result<LedgersResponse, Error> {
    let mut params: Vec<(&str, &str)> = vec![];
    if let Some(val) = asset {
        params.push(("asset", &val));
    }
    if let Some(val) = aclass {
        params.push(("aclass", &val));
    }
    if let Some(val) = type_ {
        params.push(("type", &val));
    }
    let start_string;
    if let Some(val) = start {
        start_string = val.to_string();
        params.push(("start", &start_string));
    }
    let end_string;
    if let Some(val) = end {
        end_string = val.to_string();
        params.push(("end", &end_string));
    }
    let ofs_string;
    if let Some(val) = ofs {
        ofs_string = val.to_string();
        params.push(("ofs", &ofs_string));
    }
    let response = private_request(&cred, "/0/private/Ledgers", &params).await?;
    return load_response(&response);
}

pub type QueryLedgersResponse = HashMap<String, Ledger>;

pub async fn query_ledgers(
    cred: &Credential,
    id: &[&str],
    trades: Option<bool>,
) -> Result<QueryLedgersResponse, Error> {
    let mut params: Vec<(&str, &str)> = vec![];
    let trades_string;
    if let Some(val) = trades {
        trades_string = val.to_string();
        params.push(("trades", &trades_string));
    }
    let ids = id.join(",");
    params.push(("id", &ids));
    let response = private_request(&cred, "/0/private/QueryLedgers", &params).await?;
    return load_response(&response);
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Fee {
    fee: Decimal,
    minfee: Decimal,
    maxfee: Decimal,
    nextfee: Option<Decimal>,
    nextvolume: Option<Decimal>,
    tiervolume: Decimal,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TradeVolumeResponse {
    currency: String,
    volume: Decimal,
    fees: Option<HashMap<String, Fee>>,
    fees_maker: Option<HashMap<String, Fee>>,
}

pub async fn trade_volume(
    cred: &Credential,
    pair: Option<&[&str]>,
    fee_info: Option<bool>,
) -> Result<TradeVolumeResponse, Error> {
    let mut params: Vec<(&str, &str)> = vec![];
    let pair_string;
    if let Some(val) = pair {
        pair_string = val.join(",");
        params.push(("pair", &pair_string));
    }
    let fee_info_string;
    if let Some(val) = fee_info {
        fee_info_string = val.to_string();
        params.push(("fee-info", &fee_info_string));
    }
    let response = private_request(&cred, "/0/private/TradeVolume", &params).await?;
    return load_response(&response);
}
