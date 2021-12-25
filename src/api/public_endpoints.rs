use super::models::*;
use super::request::*;
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
