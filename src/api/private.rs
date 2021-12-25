use super::request::*;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DefaultOnError};
use std::collections::HashMap;

pub async fn balance(cred: &Credential) -> Result<HashMap<String, Decimal>, Error> {
    let response = private_request(&cred, "/0/private/Balance", &[]).await?;
    return load_response(&response);
}

pub async fn balance_ex(cred: &Credential) -> Result<HashMap<String, BalanceEx>, Error> {
    let response = private_request(&cred, "/0/private/BalanceEx", &[]).await?;
    return load_response(&response);
}

pub async fn trade_balance(
    cred: &Credential,
    asset: &str,
) -> Result<HashMap<String, Decimal>, Error> {
    let response = private_request(&cred, "/0/private/TradeBalance", &[("asset", asset)]).await?;
    return load_response(&response);
}

pub async fn open_orders(
    cred: &Credential,
    trades: bool,
    userref: Option<u32>,
) -> Result<OpenOrders, Error> {
    let trades = trades.to_string();
    let mut params: Vec<(&str, &str)> = vec![("trades", &trades)];
    let userref_string;
    if let Some(val) = userref {
        userref_string = val.to_string();
        params.push(("userref", &userref_string));
    }
    let response = private_request(&cred, "/0/private/OpenOrders", &params).await?;
    return load_response(&response);
}

pub async fn closed_orders(
    cred: &Credential,
    trades: bool,
    userref: Option<u32>,
    start: Option<i64>,
    end: Option<i64>,
    ofs: Option<i64>,
    closetime: Option<String>,
) -> Result<ClosedOrders, Error> {
    let trades = trades.to_string();
    let mut params: Vec<(&str, &str)> = vec![("trades", &trades)];
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

pub async fn query_orders(
    cred: &Credential,
    trades: bool,
    userref: Option<u32>,
    txid: &str,
) -> Result<HashMap<String, Order>, Error> {
    let trades = trades.to_string();
    let mut params: Vec<(&str, &str)> = vec![("trades", &trades), ("txid", txid)];
    let userref_string;
    if let Some(val) = userref {
        userref_string = val.to_string();
        params.push(("userref", &userref_string));
    }
    let response = private_request(&cred, "/0/private/QueryOrders", &params).await?;
    return load_response(&response);
}

pub async fn trades_history(
    cred: &Credential,
    type_: &str,
    trades: bool,
    start: Option<i64>,
    end: Option<i64>,
    ofs: Option<i64>,
) -> Result<TradesHistory, Error> {
    let trades = trades.to_string();
    let mut params: Vec<(&str, &str)> = vec![("trades", &trades), ("type", type_)];
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

pub async fn query_trades(
    cred: &Credential,
    txids: &[&str],
    trades: bool,
) -> Result<HashMap<String, Trade>, Error> {
    let trades = trades.to_string();
    let mut params: Vec<(&str, &str)> = vec![("trades", &trades)];
    let txids = txids.join(",");
    if txids != "" {
        params.push(("txid", &txids))
    }
    let response = private_request(&cred, "/0/private/QueryTrades", &params).await?;
    return load_response(&response);
}

pub async fn open_positions(
    cred: &Credential,
    txids: &[&str],
    docalcs: bool,
    consolidation: &str,
) -> Result<HashMap<String, OpenPosition>, Error> {
    let docalcs = docalcs.to_string();
    let mut params: Vec<(&str, &str)> =
        vec![("docalcs", &docalcs), ("consolidation", &consolidation)];
    let txids = txids.join(",");
    if txids != "" {
        params.push(("txid", &txids))
    }
    let response = private_request(&cred, "/0/private/OpenPositions", &params).await?;
    return load_response(&response);
}

// TODO:
// /0/private/ txid=[] docalcs=bool consolidation=market
// /0/private/Ledgers asset=[] aclass="currency" type=""all"|"deposit"|"withdrawal"|"trade"|"margin"" start=int end=int ofs=int
// /0/private/QueryLedgers id=[] trades=false
// /0/private/TradeVolume pair=string

#[derive(Debug, Serialize, Deserialize)]
pub struct BalanceEx {
    balance: Decimal,
    hold_trade: Decimal,
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
pub struct OpenOrders {
    open: HashMap<String, Order>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClosedOrders {
    closed: HashMap<String, Order>,
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
pub struct TradesHistory {
    trades: HashMap<String, Trade>,
    count: u64,
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
